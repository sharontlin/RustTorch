use std::fmt::Debug;
use std::fs::File;

use crate::server::util::color::Color;
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            username: "username",
            path_to_pass = "../../password.txt",
            url = "https://nce.org/remote.php/dav",
            test_image = "test_image.png",
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
