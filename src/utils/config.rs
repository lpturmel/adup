use std::fs::{write, File};
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub game_location: String,
    pub last_login: u64,
    pub addons: Vec<Addon>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_location: "".to_string(),
            last_login: 0,
            addons: vec![],
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Addon {
    pub version: String,
    pub folders: Vec<String>,
    pub name: String,
}
impl Config {
    pub fn get_game_location(&self) -> &String {
        &self.game_location
    }
    pub fn get_last_login(&self) -> &u64 {
        &self.last_login
    }
    pub fn get_addons(&self) -> &Vec<Addon> {
        &self.addons
    }
}

const DEFAULT_CONFIG: &'static str = "./config.json";
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new(DEFAULT_CONFIG);
    if !path.exists() {
        create_config()?;
    }
    let file = File::open(Path::new(DEFAULT_CONFIG))?;

    let reader = BufReader::new(file);

    let content = serde_json::from_reader(reader)?;

    Ok(content)
}
pub fn update_config(config: &Config) -> std::io::Result<()> {
    let config_str = serde_json::to_string(config)?;

    write(DEFAULT_CONFIG, config_str)
}
fn create_config() -> std::io::Result<()> {
    let default_path = Path::new(DEFAULT_CONFIG);

    if !default_path.exists() {
        let default_config = Config {
            last_login: 0,
            game_location: "".to_string(),
            addons: vec![],
        };
        let content = serde_json::to_string(&default_config)?;
        write(Path::new(DEFAULT_CONFIG), content)?;
    }
    Ok(())
}
