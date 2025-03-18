mod settings_manager;

use settings_manager::*;
use std::{
    self, any::Any, fs::{self, File}, io::{self, prelude::*}, path::{self, Path, PathBuf}, string::{self, FromUtf8Error}
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

fn set_path (path: String, home_manager: &bool) -> Result<PathBuf, std::io::Error>{

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

fn add_package (package_name: String, home_manager: &bool) -> io::Result<()>{
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
                package_index = &index_of_file + 4;
                package_list_position = index_of_file + 4;
                loop {
                    println!("{}", &file[package_index]);
                    if file[package_index] == "];" {
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

        dbg!(file);

        Ok(())
    } else {

        path = load_settings().path_to_home_manager_config;
        unencoded_raw_file= fs::read(&path)?;
        raw_file = String::from_utf8(unencoded_raw_file).expect("Couldnt read file as Utf8");
        file = raw_file.split_whitespace().collect();

        dbg!(file);

        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    if args.init != *("None") {

        let output = create_package_file(args.init.clone(), args.home_manager);

        print!("{:?}", output);

    } else {

        if args.path != *("None"){
            let output_set_path= set_path(args.path, &args.home_manager);

            let _ = dbg!(output_set_path);
        }
        if args.install != *("None") {
            println!("Your installing the package: {}", &args.install );
            let output_add_package = add_package(args.install, &args.home_manager);

            let _ = dbg!(output_add_package);
        };
    }
    if args.debug {
        dbg!(load_settings());
    }
}
