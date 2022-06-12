use clap::Parser;

mod commands;
mod utils;
use utils::api::client::Api;

use crate::utils::config::{load_config, update_config, Config};

use self::commands::entry::{Cli, Commands};
use self::utils::fs::download_elvui;

const AUTH_SERVICE: &'static str = "auth.adup.com";
const AUTH_NAME: &'static str = "AdUp";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Install(args) => match args.name.as_str() {
            "elvui" => {
                let elvui = download_elvui().await?;

                let config = load_config()?;

                let mut addons = config.get_addons().to_vec();
                addons.push(elvui);

                let new_config = Config {
                    game_location: config.get_game_location().into(),
                    last_login: config.get_last_login().clone(),
                    addons,
                };
                update_config(&new_config)?;
            }
            _ => {}
        },
        Commands::Login(args) => match keytar::set_password(AUTH_SERVICE, AUTH_NAME, &args.pat) {
            Ok(_) => println!("Successfully logged in!"),
            Err(e) => println!("Error while logging in: {e}"),
        },
        Commands::Account => {
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
    }
    Ok(())
}
