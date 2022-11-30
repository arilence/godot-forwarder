use std::{
    fs::{self},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    pub steamapps_directory: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            steamapps_directory: PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps"),
        }
    }
}

impl AppConfig {
    pub fn load_or_create(path: PathBuf) -> Self {
        let config = match Self::load_from_file(&path) {
            Some(loaded_config) => loaded_config,
            None => {
                let new_config = AppConfig::default();
                fs::write(&path, toml::to_string_pretty(&new_config).unwrap()).unwrap();
                return new_config;
            }
        };
        return config;
    }

    fn load_from_file(path: &PathBuf) -> Option<Self> {
        let file_contents = fs::read_to_string(path).unwrap_or_default();
        match toml::from_str(&file_contents) {
            Ok(config) => Some(config),
            Err(_) => None,
        }
    }
}
