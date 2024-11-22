use std::{path::PathBuf, process::Command};
use crate::version::Version;

pub struct Package {
	pub name: String,
	pub version: Version,
	pub arch: String,
	pub maintainer: String,
	pub description: String,
	pub deps: String,
	pub path: PathBuf
}

pub fn cmd(s: &str) -> bool {
    Command::new("bash")
        .arg("-c")
        .arg(s)
        .status()
        .expect(format!("Failed to run command {}", s).as_str())
        .success()
}