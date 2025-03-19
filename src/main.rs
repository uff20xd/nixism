mod settings_manager;
mod file_manager;

use file_manager::{test_for_file_existence, write_to_packagefile};
use settings_manager::*;
use std::{
    self, fs::{self, File}, io::{self, prelude::*}, path::{self, Path, PathBuf}, process::{self, Command}
};
use clap::{command, Parser};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, default_value_t = ("None").to_owned() )]
    install: String,

    #[arg(short, long, default_value_t = 1 )]
    remove: u8,

    #[arg(long="init", default_value_t = ("None").to_owned() )]
    init: String,

    #[arg(long, default_value_t = ("None").to_owned() )]
    path: String,

    #[arg(short = 'm', default_value_t = false)]
    home_manager: bool,

    #[arg(long, short, default_value_t = false)]
    debug: bool

}

// accessed by the --init flag 
fn create_package_file (path: String, home_manager: bool) -> std::io::Result<()> {
    if !home_manager {
        let file_name = "/nixism_nixos.nix";
        let relative_path= path + file_name;
        let file_path= path::absolute(&relative_path)?;
        let mut file = File::create(&relative_path)?;

        manage_nixos_path( file_path.into_os_string().into_string().expect("huh")); 

        file.write_all(b"{ pkgs, ... }: {
environment.systemPackages = with pkgs; [

];
nix.setting.experimental-features = [ \"nix command\" \"flakes\" ];
}
")?;
    } else {
        let file_name = "/nixism_home_manager.nix";
        let relative_path = path + file_name;
        let file_path= path::absolute(&relative_path)?;
        let mut file = File::create(&relative_path)?;

        manage_home_manager_path( file_path.into_os_string().into_string().expect("huh")); 

        file.write_all(
            b"{ pkgs, ... }: {
home.packages = with pkgs; [

];
nix.setting.experimental-features = [  \"nix command\" \"flakes\"];
}
")?;
        
    }
    Ok(())
}

fn set_path (path: String, home_manager: bool) -> Result<PathBuf, std::io::Error>{

    let file_path_raw= path::absolute(&path)?;
    let file_path = file_path_raw.clone().into_os_string().into_string()
        .expect("couldnt get correct path buffer");

    if !home_manager && Path::new(&file_path).exists() {
        manage_nixos_path(file_path);

        Ok(file_path_raw)

    } else if Path::new(&file_path).exists(){
        manage_home_manager_path(file_path);

        Ok(file_path_raw)

    } else {
        print!("File not Found");

        Ok(file_path_raw)

    }
}

fn add_package (package_name: String, home_manager: bool) -> io::Result<()>{
    let path: String;
    let unencoded_raw_file: Vec<u8>;
    let raw_file: String;
    let mut file: Vec<&str>;
    let mut package_index: usize;
    let mut package_list_position: usize = 0;
    let mut installed_packages: Vec<&str> = vec!("hello");
    let mut already_installed: bool = false;
    if !home_manager {

        path = load_settings().path_to_nixos_config;
        unencoded_raw_file = fs::read(path)?;
        raw_file = String::from_utf8(unencoded_raw_file).expect("Couldnt read file as Utf8");
        file = raw_file.split_whitespace().collect();
        

        for index_of_file in 0..(&file.len() - 4) {
            println!("{:?}", &file[index_of_file..(index_of_file + 5)]);
            if file[index_of_file..(index_of_file + 5)] == ["environment.systemPackages", "=", "with", "pkgs;", "["] {
                package_index = &index_of_file + 5;
                package_list_position = index_of_file + 5;
                loop {
                    println!("{}", &file[package_index]);
                    if file[package_index] == "];" || package_index >= file.len() {
                        break;
                    }
                    installed_packages.push(file[package_index]);
                    package_index += 1;
                }
            }
        }

        for i_packages in installed_packages {
            if i_packages == package_name {
                already_installed = true;
            }
        }

        if !already_installed && (package_list_position != 0){
            file.insert(package_list_position, &package_name);
        }

        let _ = write_to_packagefile(file, home_manager);


        Ok(())
    } else {

        path = load_settings().path_to_home_manager_config;
        unencoded_raw_file= fs::read(&path)?;
        raw_file = String::from_utf8(unencoded_raw_file).expect("Couldnt read file as Utf8");
        file = raw_file.split_whitespace().collect();

        for index_of_file in 0..(&file.len() - 4) {
            if file[index_of_file..(index_of_file + 5)] == ["home.packages", "=", "with", "pkgs;", "["] {
                package_index = &index_of_file + 5;
                package_list_position = index_of_file + 5;
                loop {
                    if file[package_index] == "];" || package_index >= file.len() {
                        break;
                    }
                    installed_packages.push(file[package_index]);
                    package_index += 1;
                }
            }
        }

        for i_packages in installed_packages {
            if i_packages == package_name {
                already_installed = true;
            }
        }

        if !already_installed && (package_list_position != 0){
            file.insert(package_list_position, &package_name);
        }

        let _ = write_to_packagefile(file, home_manager);


        Ok(())
    }
}

fn rebuild (home_manager: bool) {
    let settings = load_settings();
    let mut path_to_directory: Vec<&str> = match home_manager {
        false => settings.path_to_nixos_config.split("/").collect(),
        true => settings.path_to_home_manager_config.split("/").collect(),
    };
    let _ = path_to_directory.remove(&path_to_directory.len() - 1);
    let args = path_to_directory.join("/");

    if !home_manager {
        let _output = Command::new("nixos-rebuild").arg("switch --flake");
    }
}

fn main() {
    let args = Args::parse();
    if args.init != *("None") {

        let output = create_package_file(args.init.clone(), args.home_manager);

        print!("{:?}", output);

    } else {
        let _all_fine = test_for_file_existence(args.home_manager);

        if args.path != *("None"){
            print!("Your setting path to: {}", &args.path);
            let output_set_path= set_path(args.path, args.home_manager);

        }
        if args.install != *("None") {
            println!("Your installing the package: {}", &args.install );
            let output_add_package = add_package(args.install, args.home_manager);
        };
    }
    if args.debug {
        dbg!(load_settings());
    }
}
