use anyhow::{bail, Context};
use clap::Parser;

mod commands;
mod utils;
use utils::api::client::Api;

use crate::utils::config::Config;
use std::path::Path;

use self::commands::config::command::{ConfigCommands, LocationCommands};
use self::commands::entry::{Cli, Commands};
use self::utils::fs::download_elvui;
use self::utils::message::{Message, StdErr, StdOut};

const AUTH_SERVICE: &'static str = "auth.adup.com";
const AUTH_NAME: &'static str = "AdUp";

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        StdErr::error(&e.to_string());
    }
}
async fn run() -> anyhow::Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Install(args) => {
            match args.name.as_str() {
                "elvui" => {
                    let cfg = confy::load::<Config>("adup")?;

                    if cfg.get_game_location() == "" {
                        bail!("No game location set in config! Use command config location set to set it");
                    }
                    let elvui = download_elvui().await?;

                    let mut addons = cfg.get_addons().to_vec();
                    addons.push(elvui);

                    let new_config = Config {
                        game_location: cfg.get_game_location().into(),
                        last_login: cfg.get_last_login().clone(),
                        addons,
                    };
                    confy::store("adup", new_config)?;
                }
                _ => {}
            }
        }
        Commands::Login(args) => {
            keytar::set_password(AUTH_SERVICE, AUTH_NAME, &args.pat)
                .context("Could not set the PAT")?;
            StdOut::success("Successfully logged in to Github!");
            StdOut::info("Use adup account to see info about the logged in account!");
        }
        Commands::Account => {
            let token = keytar::get_password(AUTH_SERVICE, AUTH_NAME);
            match token {
                Ok(pat) => {
                    let api = Api::new(pat.password);
                    let profile = api.get_profile().await?;

                    StdOut::info(&format!(
                        "Name: {}\nProfile url: {}",
                        profile.name, profile.html_url
                    ));
                }
                Err(_) => {
                    StdErr::error("You are not authenticated, please use the login command to set up your credentials...");
                }
            }
        }
        Commands::Config(config) => {
            let cfg = confy::load::<Config>("adup")?;
            let config_cmd = config.commands.as_ref().unwrap();
            match config_cmd {
                ConfigCommands::Location(location) => {
                    let location_cmd = &location.commands.as_ref().unwrap();
                    match location_cmd {
                        LocationCommands::Get => {
                            let current_loc = cfg.get_game_location();
                            StdOut::info(&format!("Current game location: {}", current_loc));
                        }
                        LocationCommands::Set(args) => {
                            let new_path = Path::new(&args.path);

                            if !new_path.exists() {
                                bail!("Specified path does not exist");
                            }
                            confy::store(
                                "adup",
                                Config {
                                    addons: cfg.addons,
                                    last_login: cfg.last_login,
                                    game_location: new_path.to_str().unwrap().to_string(),
                                },
                            )?;

                            StdOut::success(&format!(
                                "Successfully set the game path to: {}",
                                args.path
                            ))
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
