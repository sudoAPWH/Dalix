use std::process::Command;

pub fn cmd(s: &str) -> bool {
	Command::new("bash")
		.arg("-c")
		.arg(s)
		.status()
		.expect(format!("Failed to run command {}", s).as_str())
		.success()
}