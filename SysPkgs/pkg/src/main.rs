#![allow(unused_imports)] // ;D
#![allow(dead_code)]
#![allow(unused_variables)]

use log::{error, warn, info, debug};
use clap::Parser;
use std::path::Path;

mod version;
mod debian_utils;
mod system;

use debian_utils::{DebFile};

#[derive(Parser)]
struct Args {
    #[clap(short = 'p', long = "output-only", action = clap::ArgAction::SetTrue, help = "Only output command to be run")]
    output_only: bool,

    #[clap(short = 'r', long = "root", help = "The root directory of the system", default_value = "/")]
    root: String,

    #[clap(help = "The command to run", name = "command", index = 1)]
    command: String,

    #[clap(help = "The argument to the command", name = "arg", index = 2, required = true)]
    arg: String,

}

fn main() {
    colog::init();
    let args = Args::parse();
    let command = args.command;
    let arg = args.arg;

    info!("Command given: {}\nExcellent choice!", command);

    if args.output_only {
        info!("You have requested for output-only mode...");
    }

    if command == "xf" {
        debian_utils::extract_deb_full(
            &DebFile::new(&arg),
            Path::new(&format!("{}-dir", arg)));
    } else if command == "x" {
        debian_utils::extract_deb(
            &DebFile::new(&arg),
            Path::new(&format!("{}-dir", arg)));
    } else if command == "xi" {
        debian_utils::extract_info(&DebFile::new(&arg));
    }

}
