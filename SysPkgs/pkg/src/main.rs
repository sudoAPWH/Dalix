#[allow(unused_imports)] // ;)
use log::{error, warn, info, debug};

mod version;
mod debian_utils;
mod system;

fn main() {
	colog::init();
	info!("Logging works!");
}
