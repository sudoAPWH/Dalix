use std::path::Path;
use crate::system;
use std::process::ExitStatus;

struct DebPkg {
	name: String,
	version: String,
	arch: String,
	deps: String,
	description: String,
	maintainer: String,
	path: Path
}

pub fn extract_deb_full(d: &DebPkg, out: &Path) -> bool {
	system::cmd(&format!("dpkg-deb -R {} {}", d.path.display(), out.display()))
}

pub fn extract_deb(d: &DebPkg, out: &Path) -> bool {
	system::cmd(&format!("dpkg-deb -R {} {}", d.path.display(), out.display()))
}

///
pub fn extract_info(path: &str) -> DebPkg {

}