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
    pub published_at: String,
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
