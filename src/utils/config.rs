use std::fs::{write, File};
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    game_location: String,
    last_login: u64,
}

impl Config {}

const DEFAULT_CONFIG: &'static str = "./config.json";
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open(Path::new(DEFAULT_CONFIG))?;

    let reader = BufReader::new(file);

    let content = serde_json::from_reader(reader)?;

    Ok(content)
}
pub fn create_config() -> std::io::Result<()> {
    let default_config = Config {
        last_login: 0,
        game_location: "".to_string(),
    };
    let content = serde_json::to_string(&default_config).unwrap();
    write(Path::new(DEFAULT_CONFIG), content)
}
