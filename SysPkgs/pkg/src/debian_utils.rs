use std::path::Path;
use crate::system;
use std::process::ExitStatus;

pub struct DebPkg {
	name: String,
	version: String,
	arch: String,
	deps: String,
	description: String,
	maintainer: String,
	path: Path
}

pub struct DebFile<'a> {
	path: &'a Path
}

impl DebFile<'_> {
	pub fn new(path: &str) -> DebFile {
		DebFile {
			path: Path::new(path)
		}
	}
}

/// Extracts the full contents of a .deb package to the specified directory.
///
/// This function uses the `dpkg-deb` command to extract the entire contents of
/// the provided .deb package (`d`) into the directory specified by `out`.
///
/// # Arguments
///
/// - `d` - A reference to a `DebPkg` struct representing the .deb package to extract.
/// - `out` - A reference to a `Path` specifying the output directory where the package contents will be extracted.
///
/// # Returns
///
/// A boolean indicating the success (`true`) or failure (`false`) of the extraction process.
pub fn extract_deb_full(d: &DebPkg, out: &Path) -> bool {
	system::cmd(&format!("dpkg-deb -R {} {}", d.path.display(), out.display()))
}


pub fn extract_deb(d: &DebPkg, out: &Path) -> bool {
	system::cmd(&format!("dpkg-deb -R {} {}", d.path.display(), out.display()))
}

/// Gets a DebPkg struct from a deb file
pub fn extract_info(deb: &DebFile) {
	
}