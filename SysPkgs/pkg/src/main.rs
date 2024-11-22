#![allow(unused_imports)] // ;D
#![allow(dead_code)]
#![allow(unused_variables)]

use log::{error, warn, info, debug};

mod version;
mod debian_utils;
mod system;

fn main() {
	colog::init();
	info!("Logging works!");
}
