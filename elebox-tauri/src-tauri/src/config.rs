use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml;

const CONFIG_FILENAME: &str = "config.yaml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub language: Option<String>,
    pub database: Option<String>,
}

pub fn load_config(dir: &str) -> Result<Config, ()> {
    let mut cfg_path = PathBuf::from(dir);
    cfg_path.push(CONFIG_FILENAME);

    // TODO remove unwrap
    let content = fs::read_to_string(cfg_path).unwrap();
    let config = serde_yaml::from_str(&content).unwrap();
    Ok(config)
}

pub fn save_config(dir: &str, config: &Config) -> Result<(), ()> {
    let mut cfg_path = PathBuf::from(dir);
    cfg_path.push(CONFIG_FILENAME);

    // TODO remove unwrap
    let content = serde_yaml::to_string(config).unwrap();
    fs::write(cfg_path, content).unwrap();
    Ok(())
}

pub fn create_config(dir: &str) -> Result<(), ()> {
    let mut cfg_path = PathBuf::from(dir);
    cfg_path.push(CONFIG_FILENAME);

    if !cfg_path.exists() {
        let default_config = Config {
            language: Some("en".to_string()),
            database: None,
        };
        save_config(dir, &default_config);
    }
    Ok(())
}
