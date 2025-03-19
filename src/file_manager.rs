use core::panic;
use std::{
    fs::{self, File},
    fmt::write,
    io::Write,
    path::Path,
    error::Error,
};
use crate::settings_manager::*;


pub fn write_to_packagefile (input: Vec<&str>, home_manager: &bool) -> Result<(), Box<dyn std::error::Error>> {
    if !home_manager {
        let path = load_settings().path_to_nixos_config;
        let mut file = File::create(path)?;

        write!(file, "{}", input.join(" "))?;

        Ok(())
    }
    else {
        let path = load_settings().path_to_home_manager_config;
        let mut file = File::create(path)?;

        write!(file, "{}", input.join(" "))?;

        Ok(())
    }
}

pub fn test_for_file_existence (home_manager: bool) -> Result<String, Box<dyn std::error::Error>>{
    let file_path = load_settings();
    if !home_manager && Path::new(&file_path.path_to_home_manager_config).exists() {
        Ok("Works Fine".to_owned())
    }
    else if Path::new(&file_path.path_to_home_manager_config).exists() {
        Ok("Works Fine Aswell".to_owned())
    }
    else {
        panic!("The File doesnt exist - in function test_for_file_existence ")
    }
}
