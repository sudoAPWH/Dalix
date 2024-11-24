use core::panic;
use std::path::Path;
use std::env::consts::ARCH;

use log::{error, info, warn, debug};

use crate::system;

/// A struct representing a package source
/// source_type: e.g. deb
/// url: e.g. http://deb.debian.org/debian
/// dist: e.g. "stable" or "sid"
/// subtype: e.g. "main" or "non-free"
pub struct PackageSource {
	source_type: String,
	url: String,
	dist: String,
	subtype: String
}

/// Reads /etc/apt/sources.list and returns a Vec of PackageSources
/// The sources.list file is of the format:
/// source_type url dist subtype
/// For example:
/// deb http://deb.debian.org/debian stable main
///
/// This function will panic if the file cannot be read
pub fn read_apt_sources_list(root: &Path) -> Vec<PackageSource> {
	let mut sources: Vec<PackageSource> = Vec::new();

	let sources_list = std::fs::read_to_string(root.join("etc").join("apt").join("sources.list"));
	if let Ok(sources_list) = sources_list {
		// parse string into Vec<PackageSource>
		for line in sources_list.lines() {
			let parts = line.split(" ").collect::<Vec<&str>>();
			if parts.len() != 4 {
				continue
			}
			sources.push(PackageSource {
				source_type: parts[0].to_string(),
				url: parts[1].to_string(),
				dist: parts[2].to_string(),
				subtype: parts[3].to_string()
			})
		}
	} else {
		error!("Failed to read {}/etc/apt/sources.list", root.display());
		panic!("Failed to read {}/etc/apt/sources.list", root.display());
	}

	sources
}

pub fn update_package_lists(root: &Path) -> Result<(), String> {
	let sources = read_apt_sources_list(root);

	let pkg_cache = root.join("System").join("Cache").join("Packages");
	let pkg_cache_bak = root.join("System").join("Cache").join("Packages.bak");

	let mut i = 0;
	system::rm(&pkg_cache_bak)?;
	let _ = system::copy_recursive(
		&pkg_cache,
		&pkg_cache_bak
	); // If this fails it just means we don't have a package cache currently.
	system::rm(&pkg_cache)?;
	system::mkdir(&pkg_cache)?;

	for source in sources {
		if source.source_type == "deb" {

			let out = system::wget(
				&format!("{}/dists/{}/{}/binary-{}/Packages.gz", &source.url, &source.dist, source.subtype, system::get_arch()),
				&pkg_cache.join(i.to_string() + ".gz"));
			if let Err(e) = out {
				error!("{}", e);
				system::rm(&pkg_cache)?;
				system::copy_recursive(
					&pkg_cache_bak,
					&pkg_cache
				)?;
				return Err(e);
			}
			system::gzip_extract(&pkg_cache.join(i.to_string() + ".gz"))?;

			i += 1;
		}
	}

	Ok(())
}