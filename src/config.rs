use std::{
    fs::{self},
    path::PathBuf,
};

use directories::ProjectDirs;
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
    pub fn get_path() -> PathBuf {
        let project_dirs = ProjectDirs::from("", "", "Godot Forwarder")
            .expect("Couldn't generate config file path");
        let mut path = project_dirs.config_dir().to_path_buf();
        path.set_extension("toml");
        return path;
    }

    pub fn load_or_create(path: PathBuf) -> Self {
        let config = Self::load_from_file(&path).unwrap_or_else(|| {
            let new_config = AppConfig::default();
            fs::create_dir_all(&path.parent().unwrap()).expect("Could not create config directory");
            fs::write(&path, toml::to_string_pretty(&new_config).unwrap()).unwrap();
            return new_config;
        });
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
