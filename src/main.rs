use clap::{arg, Command};
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::time::Instant;

mod utils;
use reqwest;
use utils::api::client::Api;

const AUTH_SERVICE: &'static str = "auth.adup.com";
const AUTH_NAME: &'static str = "AdUp";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Adup")
        .about("Manager for Wow addons")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("account")
                .about("Get information about the current authenticated account"),
        )
        .subcommand(
            Command::new("login")
                .about("Log in to the github account")
                .arg(arg!(<PAT> "The PAT to use"))
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("account", _)) => {
            let token = keytar::get_password(AUTH_SERVICE, AUTH_NAME);
            match token {
                Ok(pat) => {
                    let api = Api::new(pat.password);
                    let profile = api.get_profile().await?;

                    println!("You are logged in as: {}", profile.name);
                }
                Err(_) => {
                    println!("You are not authenticated, please use the login command to set up your credentials...");
                }
            }
        }
        Some(("login", sub_matches)) => {
            match keytar::set_password(
                AUTH_SERVICE,
                AUTH_NAME,
                sub_matches.value_of("PAT").unwrap(),
            ) {
                Ok(_) => println!("Successfully logged in!"),
                Err(e) => println!("Error while logging in: {e}"),
            }
        }
        Some((value, _)) => {
            println!("{value} is not a valid command!");
        }
        None => {
            unreachable!()
        }
    }
    // let client = reqwest::Client::new();
    //
    // // let res = client.get("https://api.github.com/user")
    // //     .send().await?;
    // // let json = res.text().await?;
    // // println!("{json:?}");
    // let res = client
    //     .get("https://www.tukui.org/download.php?ui=elvui")
    //     .send()
    //     .await?;
    // let text = res.text().await?;
    // if let Some(i) = text.find("/downloads/elvui-") {
    //     let version = text[i..i + 26].to_string();
    //
    //     let tukui_url = "https://www.tukui.org";
    //     let download_url = format!("{tukui_url}{version}");
    //     download(download_url).await?;
    // }

    Ok(())
}

struct Addon {
    version: String,
    name: String,
    src_url: String,
}
struct ElvUi {
    version: String,
}
async fn download_remote_file(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let response = reqwest::get(&url).await?;
    let file_name = url.split("/").last().unwrap();
    let path_name = format!("./{file_name}");

    let path = Path::new(&path_name);
    let mut file = File::create(&path)?;

    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    let elapsed = now.elapsed().as_millis();
    println!("Finished downloading: {file_name} in {elapsed}ms");
    Ok(())
}
