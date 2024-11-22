use std::process::Command;

pub struct Package {
	pub name: String,
	pub version: String
}

pub fn cmd(s: &str) -> bool {
    Command::new("bash")
        .arg("-c")
        .arg(s)
        .status()
        .expect(format!("Failed to run command {}", s).as_str())
        .success()
}