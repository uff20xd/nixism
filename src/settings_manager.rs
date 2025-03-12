use serde_derive::*;
use users::*;
use std::{
    fs,
    fs::File,
    io::prelude::*,
    path::*,
};



#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub path_to_nixos_config: String,
    pub path_to_home_manager_config: String,
}


pub fn load_settings() -> Config {
    let homedirectory= get_home_directory();
    let settings_file: String;

    // if the file exists it just reads from it
    if Path::new(&(homedirectory.clone() + "/.config/nixism/config.toml")).exists() {
        settings_file = fs::read_to_string( homedirectory.clone() + "/.config/nixism/config.toml")
            .expect("Couldnt read the settings file");

        toml::from_str(&settings_file)
            .expect("couldnt Deserialize the Config object")
    }

    // If the File doesnt exist yet it will be created 
    else {

        let _ = fs::create_dir(homedirectory.clone() + "/.config/nixism");

        let config = Config{path_to_nixos_config: ("None").to_string(), path_to_home_manager_config: ("None").to_string()};

        let mut new_config_file = File::create(homedirectory.clone() + "/.config/nixism/config.toml")
            .expect("new config file in load setting");

        let new_config_file_contents = toml::to_string(&config)
            .unwrap();


        let _ = write!(new_config_file, "{}", new_config_file_contents);

        settings_file = fs::read_to_string(homedirectory.clone() + "/.config/nixism/config.toml").expect("cool");

        toml::from_str(&settings_file)
            .expect("couldnt deserialize from file")
    }
}

fn write_settings(config_input: Config) -> std::io::Result<()>{

    let homedirectory = get_home_directory();

    let new_config_file_contents = toml::to_string(&config_input)
        .expect("couldnt Serialize to toml lol"); 

    let mut new_config_file = File::create(homedirectory.clone() + "/.config/nixism/config.toml")?;

    let _ = write!(new_config_file, "{}", new_config_file_contents);



    Ok(())

}

pub fn get_home_directory () -> String {
    let current_user_as_osstring = get_current_username()
        .expect("couldnt get username");
    let current_user = current_user_as_osstring.to_str()
        .expect("couldnt convert username to a string");
    ("/home/").to_string() + current_user
}

pub fn manage_home_manager_path(path_to_home_manager_config: String) {
    let mut config: Config = load_settings();
    config.path_to_home_manager_config = path_to_home_manager_config;
    write_settings(config).expect("You seem to suck balls if you get this error");
}

pub fn manage_nixos_path(path_to_home_manager_config: String) {
    let mut config: Config = load_settings();
    config.path_to_nixos_config = path_to_home_manager_config;
    write_settings(config).expect("You seem to suck balls if you get this error");
}


