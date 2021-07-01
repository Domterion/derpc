use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub configuration: Configuration,
    pub statuses: HashMap<String, StatusConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub client_id: String,
    pub time_between: u64,
}

#[derive(Debug, Deserialize)]
pub struct ButtonConfig {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct StatusConfig {
    pub state: String,
    pub details: String,
    pub large_image: Option<String>,
    pub small_image: Option<String>,
    pub buttons: Option<Vec<ButtonConfig>>,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        config
    }
}
