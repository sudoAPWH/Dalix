use std::{path::PathBuf, process::{Command, Output}};
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

pub fn cmd_out(s: &str) -> Output {
    Command::new("bash")
        .arg("-c")
        .arg(s)
        .output()
        .expect(format!("Failed to run command {}", s).as_str())
}

pub fn copy_recursive(from: &PathBuf, to: &PathBuf) -> bool {
	cmd(&format!("cp -r {}/* {}", from.display(), to.display()))
}