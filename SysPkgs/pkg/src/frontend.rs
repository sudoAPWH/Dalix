use std::path::Path;

use crate::system;

/// A struct representing a package source
/// source_tpye: e.g. deb
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
pub fn read_sources_list(root: &Path) -> Vec<PackageSource> {
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
		panic!("Failed to read /etc/apt/sources.list");
	}

	sources
}

pub fn update_package_lists(root: &Path) -> bool {
	let sources = read_sources_list(root);

	let mut i = 0;
	for source in sources {
		if source.source_type == "deb" {
			let rand_string =
			system::wget(
				&source.url,
				&root.join("System").join("Cache").join("Packages").join(i.to_string()));
			i += 1;
		}
	}

	true
}