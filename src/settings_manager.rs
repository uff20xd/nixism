use serde_derive::*;
use std::{default::Default, io::Error, slice::ChunksMut};
use confy;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    path_to_nixos_config: String,
    path_to_home_manager_config: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { path_to_nixos_config: "None".to_string(), path_to_home_manager_config: "None".to_string() }
    }
}

pub fn change_nixos_path() -> Result<(), confy::ConfyError> {
    let cfg: () = confy::load("nixism", None)?;
    dbg!(cfg);

    Ok(())
}

