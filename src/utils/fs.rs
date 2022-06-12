use crate::utils::config::Addon;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::cmp::min;
use std::fs::File;
use std::fs::{self, remove_file};
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

pub async fn download_elvui() -> Result<Addon, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://www.tukui.org/download.php?ui=elvui")
        .send()
        .await?;

    let text = res.text().await?;
    match text.find("/downloads/elvui-") {
        Some(i) => {
            let version = text[i..i + 26].to_string();

            let tukui_url = "https://www.tukui.org";
            let download_url = format!("{tukui_url}{version}");
            download_remote_file(download_url).await?;
            Ok(Addon {
                name: "elvui".to_string(),
                folders: vec![],
                version,
            })
        }
        None => panic!("Cannot find Elvui download path"),
    }
}
pub async fn download_remote_file(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let response = reqwest::get(&url).await?;

    let total_size = response
        .content_length()
        .ok_or("Failed to get content length for Elvui download")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    let elapsed_http = now.elapsed().as_millis();
    let file_name = url.split("/").last().unwrap();
    println!("Finished downloading: {file_name} in {elapsed_http}ms");

    let path_name = format!("/Users/lpturmel/dev/rust/adup/test/{file_name}");

    let path = Path::new(&path_name);
    let mut file = File::create(&path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }
    // let mut content = Cursor::new(response.bytes().await?);

    pb.finish_with_message(format!("Downloaded {file_name} in {elapsed_http}ms"));
    // std::io::copy(&mut content, &mut file)?;
    let target_dir = Path::new("./test");
    extract(file_name, &path, &target_dir);
    // Cleanup
    remove_file(&path)?;
    Ok(())
}

fn extract(file_name: &str, path_ref: &Path, target_dir: &Path) {
    let now = Instant::now();
    let file = fs::File::open(path_ref).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

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
    println!("Finished extracting {file_name} in {elapsed}ms");
}
