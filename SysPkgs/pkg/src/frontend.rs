use std::path::Path;

/// A struct representing a package source
/// source_tpye: e.g. deb
/// url: e.g. http://deb.debian.org/debian
/// dist: e.g. "stable" or "sid"
/// subtype: e.g. "main" or "non-free"
struct PackageSource {
	source_type: String,
	url: String,
	dist: String,
	subtype: String
}

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
	};

	sources
}

pub fn update_package_lists(root: &Path) -> bool {
	read_sources_list(root);

	true
}