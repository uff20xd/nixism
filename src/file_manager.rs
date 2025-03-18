use std::{
    fs::write,
};
use crate::settings_manager::*;


pub fn write_to_packagefile (input: Vec<&str>, home_manager: bool) {
    if !home_manager {
        let path = load_settings().path_to_nixos_config;
    }
}
