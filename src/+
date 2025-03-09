mod settings_manager;

use std::{
    process::Command,
    fs::{
        OpenOptions,
        File,
    },
    io::prelude::*,
};
use clap::{command, Parser};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, default_value_t = ("None").to_string() )]
    install: String,

    #[arg(short, long, default_value_t = 1 )]
    remove: u8,

    #[arg(long="init", default_value_t = ("None").to_string() )]
    init: String,

    #[arg(long, default_value_t = ("None").to_string() )]
    path: String,

    #[arg(short = 'm', default_value_t = false)]
    home_manager: bool,

}

fn create_package_file (path: String, home_manager: bool) -> std::io::Result<()> {
    if !home_manager {

        let mut file = File::create(path)?;
        file.write_all(b"{ pkgs, ... }: {
environment.systemPackage = with pkgs; [

];
nix.setting.experimental-features = [ \"nix command\" \"flakes\" ];
}
")?;
    } else {
        let mut file = File::create(path)?;
        file.write_all(b"{ pkgs, ... }: {
home.packages = with pkgs; [

];
nix.setting.experimental-features = [  \"nix command\" \"flakes\"];
}
")?;
        
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    if args.init != *("None") {

        let output = create_package_file(args.init.clone(), args.home_manager);

        print!("{:?}", output);

    } else {

        if args.path != *("None"){

        }
        if args.install != *("None") {
            println!("Your installing the package: {}", args.install );
        };
    }



    
}
