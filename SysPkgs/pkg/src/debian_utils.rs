use std::{fs, path::{Path, PathBuf}};
use crate::system;
use std::process::ExitStatus;
use temp_dir::TempDir;
use std::fs::File;
use log::{error, warn, info, debug};
use debian_packaging::package_version::PackageVersion;

pub struct DebPkg {
    name: String,
    version: PackageVersion,
    arch: String,
    deps: String,
    description: String,
    maintainer: String,
    path: PathBuf
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
///
/// # Example
/// ```
/// let deb = DebPkg {
///     name: "package-name".to_string(),
///     version: "1.0".to_string(),
///     arch: "amd64".to_string(),
///     deps: "libwhatever-dev".to_string(),
///     description: "Package Description".to_string(),
///     maintainer: "Package Maintainer".to_string(),
///     path: Path::new("/path/to/my-package.deb"),
/// };
///
/// if extract_deb_full(&deb, Path::new("/path/to/extracted")) {
///     println!("Extraction successful!");
/// } else {
///     println!("Extraction failed!");
/// }
/// ```
pub fn extract_deb_full(d: &DebFile, out: &Path) -> bool {
    system::cmd(&format!("dpkg-deb -R {} {}", d.path.display(), out.display()))
}

pub fn extract_deb(d: &DebFile, out: &Path) -> bool {
    system::cmd(&format!("dpkg-deb -x {} {}", d.path.display(), out.display()))
}

/// Gets a DebPkg struct from a deb file
///
/// Generates a DebPkg struct based off of information in the debian control file.
/// Also imbeds the path field.
///
/// # Arguments
/// - `deb` The `DebFile` for which will will generate the DebPkg struct for.
///
/// # Returns
/// - `DebPkg` A struct containing info about the package.
pub fn extract_info(deb: &DebFile) -> DebPkg {
    let dir = TempDir::new().unwrap();
    extract_deb_full(&deb, dir.path());
    let mut control_file = File::open(
            dir.path()
            .join("DEBIAN")
            .join("control")
        ).unwrap();
    let mut control_string = String::new();
    std::io::Read::read_to_string(&mut control_file, &mut control_string).unwrap();

    let mut name = String::new();
    let mut version = String::new();
    let mut arch = String::new();
    let mut deps = String::new();
    let mut description = String::new();
    let mut maintainer = String::new();

    let mut block: String = "".to_string();
    for line in control_string.lines() {
        if line.starts_with("Package: ") {
            block = "".to_string();
            info!("{}", line);
            line[9..].clone_into(&mut name)
        } else if line.starts_with("Version: ") {
            block = "".to_string();
            info!("{}", line);
            line[9..].clone_into(&mut version)
        } else if line.starts_with("Architecture: ") {
            block = "".to_string();
            info!("{}", line);
            line[14..].clone_into(&mut arch)
        } else if line.starts_with("Depends: ") {
            block = "".to_string();
            info!("{}", line);
            line[9..].clone_into(&mut deps)
        } else if line.starts_with("Maintainer: ") {
            block = "".to_string();
            info!("{}", line);
            line[12..].clone_into(&mut maintainer)
        } else if line.starts_with("Description: ") {
            block = "Description".to_string();
            line[13..].clone_into(&mut description)
        } else if line.starts_with(" ") {
            if block != "".to_string() {
                if block == "Description" {
                    description.push_str(&format!("{}\n",&line[1..]));
                }
            }
        }
    }
    info!("Description: {}", description);
    DebPkg {
        name,
        version: PackageVersion::parse(&version).unwrap(),
        arch,
        deps,
        description,
        maintainer,
        path: deb.path.to_path_buf()
    }
}

/// Installs a DebFile into the path specified by root
pub fn install_deb_pkg(d: &DebFile, root: &Path) -> bool{
	let info = extract_info(d);
	let pkg_dir = root.join(format!("System/Packages/{}---{}", info.name, info.version.to_string()));
	{
		let dir = TempDir::new().unwrap();
		extract_deb(&d, dir.path());
		if system::copy_recursive(
			&dir.path().to_path_buf(),
			&pkg_dir
		) {
			info!("Extracted {} to {}", info.name, pkg_dir.display());
		} else {
			panic!("Failed to extract {} to {}", info.name, pkg_dir.display());
		}
	} // TempDir gets dropped here
	// pkg_dir is stil valid though


	let pkg_info_path = pkg_dir.join("pkg-info");
	let info: String = format!(
r"InfoType = 1
DepsIncluded = false

[Package]
Name = '{}'
Version = '{}'
Architecture = '{}'
Depends = '{}'
Maintainer = '{}'
Description = '''{}'''

[Other]
source = 'deb'",
	info.name,
	info.version.to_string(),
	info.arch,
	info.deps,
	info.maintainer,
	info.description);
	info!("Pkg info:\n{}", info);
	system::touch(&pkg_info_path);
	fs::write(&pkg_info_path, info).unwrap();
	info!("Wrote pkg-info to {}", &pkg_info_path.display());

	true
}