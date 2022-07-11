use crate::utils::config::{Addon, Config};
use anyhow::Context;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::cmp::min;
use std::fs::File;
use std::fs::{self, remove_file};
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

use super::message::{Message, StdOut};

pub async fn download_elvui() -> Result<Addon, anyhow::Error> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://www.tukui.org/download.php?ui=elvui")
        .send()
        .await?;

    let text = res.text().await?;

    let version_url = text
        .lines()
        .find(|line| line.contains("Download ElvUI"))
        .map(|line| {
            let url = line
                .split("href=\"")
                .nth(1)
                .unwrap()
                .split("\"")
                .next()
                .unwrap();
            let url = url.split("\"").next().unwrap();
            url.to_string()
        })
        .ok_or_else(|| anyhow::anyhow!("Could not find download link"))?;

    let tukui_url = "https://www.tukui.org";
    let download_url = format!("{tukui_url}{version_url}");
    let extracted_dirs = download_remote_file(download_url).await?;
    let now = chrono::Utc::now();
    let version = version_url
        .split("-")
        .last()
        .unwrap()
        .to_string()
        .split(".zip")
        .next()
        .unwrap()
        .to_string();

    Ok(Addon {
        name: "elvui".to_string(),
        folders: extracted_dirs,
        published_at: now.to_string(),
        version,
    })
}
pub async fn download_remote_file(url: String) -> Result<Vec<String>, anyhow::Error> {
    let now = Instant::now();
    let response = reqwest::get(&url).await?;

    let total_size = response
        .content_length()
        .context("Failed to get content length for Elvui download")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    let elapsed_http = now.elapsed().as_millis();
    let file_name = url.split("/").last().unwrap();
    StdOut::success("Finished downloading: {file_name} in {elapsed_http}ms");

    let cfg = confy::load::<Config>("adup")?;

    let target_dir = Path::new(cfg.get_game_location());
    let tmp_path = target_dir.join(file_name);
    let mut file = File::create(&tmp_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.context(format!("Error while downloading file"))?;
        file.write_all(&chunk)
            .context(format!("Error while writing to file"))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }
    // let mut content = Cursor::new(response.bytes().await?);

    pb.finish_with_message(format!("Downloaded {file_name} in {elapsed_http}ms"));

    let extracted_dirs = extract(file_name, &tmp_path, &target_dir);

    // Cleanup
    remove_file(&tmp_path)?;
    Ok(extracted_dirs)
}

fn extract(file_name: &str, path_ref: &Path, target_dir: &Path) -> Vec<String> {
    let now = Instant::now();
    let file = fs::File::open(path_ref).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut extracted_dir: Vec<String> = vec![];

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path.to_owned()),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        if file.is_dir() {
            let parent = file.name().split("/").next().unwrap();
            let already_exists = extracted_dir.iter().any(|dir| dir == &parent.to_string());
            if !already_exists {
                extracted_dir.push(parent.to_string());
            }
        }

        if (*file.name()).ends_with('/') {
            // println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            // );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    let elapsed = now.elapsed().as_millis();
    StdOut::success(&format!(
        "Finished extracting {} in {}ms",
        file_name, elapsed
    ));
    extracted_dir
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_can_find_elvui_version() {
        let text = "<a href='/downloads/elvui-12.81.zip' class='btn btn-mod btn-border-w btn-round btn-large'>Download ElvUI 12.81</a>";

        let version_start = text.find("-").unwrap();
        let version_end = text.find("zip").unwrap();
        let version = text[version_start + 1..version_end - 1].to_string();

        assert_eq!(version, "12.81");
    }
}
