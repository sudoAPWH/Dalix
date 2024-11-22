use debian_packaging::package_version::PackageVersion;

struct Requires {
	name: String,
	version: Option<PackageVersion>,
}

struct OR {
	items: Vec<Requires>
}