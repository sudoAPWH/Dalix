struct DebInfo {
	name: String,
	version: String,
	arch: String,
	deps: String,
	description: String,
	maintainer: String,
	homepage: String,
}

pub fn extract_info(path: &str) -> DebInfo {
}