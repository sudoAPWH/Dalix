use std::{env::consts::ARCH, io::Error, path::PathBuf, process::{Command, Output}};
use debian_packaging::package_version::PackageVersion;

pub struct Package {
	pub name: String,
	pub version: PackageVersion,
	pub arch: String,
	pub maintainer: String,
	pub description: String,
	pub deps: String,
	pub path: PathBuf
}

/// Executes a shell command using bash.
///
/// This function runs the given command string `s` in a bash shell.
/// It returns a boolean indicating the success (`true`) or failure (`false`)
/// of the command execution. If the command fails to run, it panics with
/// an error message that includes the command string.
///
/// # Arguments
///
/// - `s` - A string slice that holds the command to execute.
///
/// # Returns
///
/// A boolean indicating whether the command executed successfully.
pub fn cmd(s: &str) -> Result<(), String> {
    if Command::new("bash")
        .arg("-c")
        .arg(s)
        .status()
        .expect(format!("Failed to run command {}", s).as_str())
        .success() == true {
		Ok(())
	} else {
		Err(format!("Failed to run command {}", s))
	}
}

/// Executes a shell command and captures its output.
///
/// This function uses the `bash` command to execute the shell command specified
/// by the input string `s`. It returns an `Output` struct containing the standard
/// output and standard error of the executed command. If the command fails to run,
/// it will panic with an error message that includes the command string.
///
/// # Arguments
///
/// - `s` - A string slice that holds the shell command to be executed.
///
/// # Returns
///
/// An `Output` struct containing the results of running the command.
pub fn cmd_out(s: &str) -> Result<Output, Error> {
    Command::new("bash")
        .arg("-c")
        .arg(s)
        .output()
}

pub fn get_arch() -> String {
	let arch = ARCH;
	if arch == "x86_64" {
		"amd64".to_string()
	} else if arch == "aarch64" {
		"aarch64".to_string()
	} else { // WHAT!!!!
		"amd64".to_string()
	}
}

/// Recursively copies the contents of one directory to another.
///
/// This function uses the `mkdir -p` command to create the destination directory and all its parents if they
/// do not already exist, and then the `cp -r` command to copy the contents of the source directory to the destination
/// directory.
pub fn copy_recursive(from: &PathBuf, to: &PathBuf) -> Result<(), String> {
	mkdir(to)?;
	cmd(&format!("cp -r {}/* {}", from.display(), to.display()))
}

/// Creates a new directory at the specified path.
///
/// This function uses the `mkdir -p` command to create the directory and all its parents if they
/// do not already exist.
pub fn mkdir(path: &PathBuf) -> Result<(), String> {
	cmd(&format!("mkdir -p {}", path.display()))
}

/// Creates an empty file or updates the access and modification timestamps of an existing file
/// at the specified path.
///
/// This function uses the `touch` command to create a new file or update the timestamps of an
/// existing file at the given path.
pub fn touch(path: &PathBuf) -> Result<(), String> {
	cmd(&format!("touch {}", path.display()))
}

/// Downloads the given URL and saves the contents to the given path.
///
/// This function uses the `wget` command to download the given URL, and saves the contents to the given path.
pub fn wget(url: &str, out: &PathBuf) -> Result<(), String> {
	cmd(&format!("wget {} -O {}", url, out.display()))
}

/// Deletes the file or directory at the given path.
///
/// This function uses the `rm -rf` command to delete the given path.
pub fn rm(path: &PathBuf) -> Result<(), String> {
	cmd(&format!("rm -rf {}", path.display()))
}

/// Decompresses a gzipped file at the given path.
///
/// This function uses the `gzip -d` command to decompress a gzipped file at the given path.
/// Note: It DOES delete the archive file.
pub fn gzip_extract(path: &PathBuf) -> Result<(), String> {
	cmd(&format!("gzip -d {}", path.display()))
}