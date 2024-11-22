#![allow(unused_imports)] // ;D
#![allow(dead_code)]
#![allow(unused_variables)]

use log::{error, warn, info, debug};
use clap::Parser;

mod version;
mod debian_utils;
mod system;

#[derive(Parser)]
struct Args {
	#[clap(short = 'p', long = "output-only", action = clap::ArgAction::SetTrue, help = "Only output command to be run")]
	output_only: bool,

	#[clap(short = 'r', long = "root", help = "The root directory of the system")]

	#[clap(help = "The command to run", name = "command", index = 1)]
	command: String,

}

fn main() {
	colog::init();
	let args = Args::parse();

	info!("Command given: {}\nExcellent choice!", args.command);

	if args.output_only {
		info!("You have requested for output-only mode...");
	}

}
