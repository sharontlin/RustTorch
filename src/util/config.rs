use std::fmt::Debug;
use std::fs::File;

use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;

// This structure describes the configuration JSON file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub username: String,
    pub path_to_pass: String,
    pub url: String,
    pub image_name: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            username: "username".to_string(),
            path_to_pass: "../../password.txt".to_string(),
            url: "https://nce.org/remote.php/dav/files/model.pt".to_string(),
            image_name: "test_image.png".to_string(),
        }
    }
}

pub static CONFIG: Lazy<Configuration> = Lazy::new(load_config);

/// If the configuration file exists, load it. Otherwise, use the default configuration.
fn load_config() -> Configuration {
    let config_path = PathBuf::from("./server.json");
    let config_file = File::open(config_path).ok();
    if let Some(config_file) = config_file {
        serde_json::from_reader(config_file).expect("Failed to parse config file")
    } else {
        info!("No config file detected, using default configuration");
        Configuration::default()
    }
}
