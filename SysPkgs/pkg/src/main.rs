#![allow(unused_imports)] // ;D
#![allow(dead_code)]
#![allow(unused_variables)]

use log::{error, warn, info, debug};
use clap::Parser;
use std::path::Path;

mod backend;
mod system;
mod dependencies;
mod frontend;

use backend::DebFile;

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
	info!("With root directory: {}", args.root);
	info!("With argument: {}", arg);

    if args.output_only {
        info!("You have requested for output-only mode...");
    }

    if command == "xf" {
        backend::extract_deb_full(
            &DebFile::new(&arg),
            Path::new(&format!("{}-dir", arg)));
    } else if command == "x" {
        backend::extract_deb(
            &DebFile::new(&arg),
            Path::new(&format!("{}-dir", arg)));
    } else if command == "install-deb" {
		backend::install_deb_pkg(
			&DebFile::new(&arg),
			Path::new(&args.root)
		);
	}

}
