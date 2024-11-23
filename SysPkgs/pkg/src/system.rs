use std::{path::PathBuf, process::{Command, Output}};
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
pub fn cmd(s: &str) -> bool {
    Command::new("bash")
        .arg("-c")
        .arg(s)
        .status()
        .expect(format!("Failed to run command {}", s).as_str())
        .success()
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
pub fn cmd_out(s: &str) -> Output {
    Command::new("bash")
        .arg("-c")
        .arg(s)
        .output()
        .expect(format!("Failed to run command {}", s).as_str())
}

/// Recursively copies the contents of one directory to another.
///
/// This function uses the `mkdir -p` command to create the destination directory and all its parents if they
/// do not already exist, and then the `cp -r` command to copy the contents of the source directory to the destination
/// directory. It returns a boolean indicating the success (`true`) or failure (`false`) of the operation.
pub fn copy_recursive(from: &PathBuf, to: &PathBuf) -> bool {
	mkdir(to);
	cmd(&format!("cp -r {}/* {}", from.display(), to.display()))
}

/// Creates a new directory at the specified path.
///
/// This function uses the `mkdir -p` command to create the directory and all its parents if they
/// do not already exist. It returns a boolean indicating the success (`true`) or failure (`false`)
/// of the operation.
pub fn mkdir(path: &PathBuf) -> bool {
	cmd(&format!("mkdir -p {}", path.display()))
}

/// Creates an empty file or updates the access and modification timestamps of an existing file
/// at the specified path.
///
/// This function uses the `touch` command to create a new file or update the timestamps of an
/// existing file at the given path. It returns a boolean indicating the success (`true`) or
/// failure (`false`) of the operation.
pub fn touch(path: &PathBuf) -> bool {
	cmd(&format!("touch {}", path.display()))
}

/// Downloads the given URL and saves the contents to the given path.
///
/// This function uses the `wget` command to download the given URL, and saves the contents to the given path.
/// It returns a boolean indicating the success (`true`) or failure (`false`) of the download operation.
pub fn wget(url: &str, out: &PathBuf) -> bool {
	cmd(&format!("wget {} -o {}", url, out.display()))
}

/// Deletes the file or directory at the given path.
///
/// This function uses the `rm -rf` command to delete the given path. It
/// returns a boolean indicating the success (`true`) or failure (`false`) of the
/// deletion operation.
pub fn rm(path: &PathBuf) -> bool {
	cmd(&format!("rm -rf {}", path.display()))
}