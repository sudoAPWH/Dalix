#[allow(unused_imports)] // ;)
use log::{error, warn, info, debug};

mod version;
mod debian_utils;

fn main() {
	colog::init();
	info!("Logging works!");
}
