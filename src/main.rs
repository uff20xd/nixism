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

    #[arg(short, long)]
    install: String,

    #[arg(short, long, default_value_t = 1 )]
    remove: u8,

    #[arg(long, default_value_t = true )]
    init: bool,

}

fn main() {
    let args = Args::parse();

    println!("Your installing the package: {}", args.install );
    
}
