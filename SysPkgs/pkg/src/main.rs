#[allow(unused_imports)] // ;)
use log::{error, warn, info, debug};

mod version;

fn main() {
	colog::init();
	info!("Logging works!");
}
