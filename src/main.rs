use clap::Parser;

mod commands;
mod utils;
use utils::api::client::Api;

use crate::utils::config::Config;
use std::path::Path;

use self::commands::config::command::{ConfigCommands, LocationCommands};
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
                let cfg = confy::load::<Config>("adup")?;

                if cfg.get_game_location() == "" {
                    panic!("Set game location first to install addons");
                }
                let elvui = download_elvui().await?;

                // let config = load_config()?;

                let mut addons = cfg.get_addons().to_vec();
                addons.push(elvui);

                let new_config = Config {
                    game_location: cfg.get_game_location().into(),
                    last_login: cfg.get_last_login().clone(),
                    addons,
                };
                confy::store("adup", new_config)?;
                // update_config(&new_config)?;
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
        Commands::Config(config) => {
            let cfg = confy::load::<Config>("adup")?;
            let config_cmd = config.commands.as_ref().unwrap();
            match config_cmd {
                ConfigCommands::Location(location) => {
                    let location_cmd = &location.commands.as_ref().unwrap();
                    match location_cmd {
                        LocationCommands::Get => {
                            let current_loc = cfg.get_game_location();
                            println!("Current game location: {current_loc}");
                        }
                        LocationCommands::Set(args) => {
                            let new_path = Path::new(&args.path);

                            if !new_path.exists() {
                                panic!("Specified path does not exist");
                            }
                            confy::store(
                                "adup",
                                Config {
                                    addons: cfg.addons,
                                    last_login: cfg.last_login,
                                    game_location: new_path.to_str().unwrap().to_string(),
                                },
                            )?;

                            println!("Successfully set the game path to: {}", args.path)
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
