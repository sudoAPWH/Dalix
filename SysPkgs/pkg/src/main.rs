#[allow(unused_imports)] // ;)
use log::{error, warn, info, debug};

fn main() {
	colog::init();
	info!("Logging works!");
}
