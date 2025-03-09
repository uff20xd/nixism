use serde::{Serialize, Deserialize};
use std::default::Default;
use confy;

#[derive(Serialize, Deserialize)]
struct Config {
    path_to_nixos_config: String,
    path_to_home_manager_config: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { path_to_nixos_config: ("None").to_string(), path_to_home_manager_config: ("None").to_string()}
    }
    
}


