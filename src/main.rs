mod settings_manager;
mod file_manager;
mod manage_packages;

use file_manager::*;
use settings_manager::*;
use manage_packages::*;
use std::{
    self, process::Command,
};
use clap::{command, error::Result, Parser};


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
    
    #[arg(long, short = 'b', default_value_t = false)]
    rebuild: bool,

    #[arg(short = 'm', default_value_t = false)]
    home_manager: bool,

    #[arg(long, short, default_value_t = false)]
    debug: bool,

    #[arg(long, short, default_value_t = false)]
    self_update: bool,

    #[arg(long,short, default_value_t = false)]
    update: bool

}


fn rebuild (home_manager: bool) -> Result<(), > {
    let settings = load_settings();
    let mut path_to_directory: Vec<&str> = match home_manager {
        false => settings.path_to_nixos_config.split("/").collect(),
        true => settings.path_to_home_manager_config.split("/").collect(),
    };
    let _ = path_to_directory.remove(path_to_directory.len() - 1);
    let args= path_to_directory.join("/");
    let mut output;

    if !home_manager {
        output = Command::new("sudo");
        output.arg("nixos-rebuild").arg("switch").arg("--flake").arg(".").current_dir(&args).output()?;
        println!("{:?}", output)
    }
    else {
        output = Command::new("home-manager");
        output.arg("switch").arg("--flake").arg(".").current_dir(&args).output()?;
        println!("{:?}", output);
    }
    Ok(())
}

fn self_update (home_manager: bool) -> Result<()> {
    
    let settings = load_settings();
    let mut path_to_directory: Vec<&str> = match home_manager {
        false => settings.path_to_nixos_config.split("/").collect(),
        true => settings.path_to_home_manager_config.split("/").collect(),
    };
    let _ = path_to_directory.remove(path_to_directory.len() - 1);
    let args= path_to_directory.join("/");
    let mut command = Command::new("nix");

    command.arg("flake").arg("update").arg("nixism").current_dir(&args).output()?;

    Ok(())
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
            let _output_set_path= set_path(args.path, args.home_manager);
        }
        if args.self_update {
            print!("Updating Nixism");
            let _output_self_update = self_update(args.home_manager);
        }

        if args.install != *("None") {
            println!("Your installing the package: {}", &args.install );
            let _output_add_package = add_package(args.install, args.home_manager);
        }
        if args.rebuild {
            let _ = rebuild(args.home_manager);
        }
    }
    if args.debug {
        dbg!(load_settings());
    }
}
