use crate::system;
use debian_packaging::package_version::PackageVersion;
use log::{debug, error, info, warn};
use std::fmt::Display;
use std::fs::File;
use std::process::ExitStatus;
use std::{
    fs,
    path::{Path, PathBuf},
};
use temp_dir::TempDir;

pub struct Pkg {
    pub name: String,
    pub version: PackageVersion,
    pub arch: String,
    pub deps: String,
    pub recommends: String,
    pub suggests: String,
    pub pre_depends: String,
    pub enhances: String,
    pub description: String,
    pub maintainer: String,
    pub homepage: String,
    pub path: PathBuf,
}

impl Display for Pkg {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}---{} {} url: {}", self.name, self.version, self.arch, self.path.display())
	}
}

pub struct DebFile<'a> {
    path: &'a Path,
}

impl DebFile<'_> {
    pub fn new(path: &str) -> DebFile {
        DebFile {
            path: Path::new(path),
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
/// - `d` - A reference to a `Pkg` struct representing the .deb package to extract.
/// - `out` - A reference to a `Path` specifying the output directory where the package contents will be extracted.
///
/// # Returns
///
/// A boolean indicating the success (`true`) or failure (`false`) of the extraction process.
///
/// # Example
/// ```
/// let deb = Pkg {
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
pub fn extract_deb_full(d: &DebFile, out: &Path) -> Result<(), String> {
    system::cmd(&format!(
        "dpkg-deb -R {} {}",
        d.path.display(),
        out.display()
    ))
}

pub fn extract_deb(d: &DebFile, out: &Path) -> Result<(), String> {
    system::cmd(&format!(
        "dpkg-deb -x {} {}",
        d.path.display(),
        out.display()
    ))
}

/// Gets a Pkg struct from a deb file
///
/// Generates a Pkg struct based off of information in the debian control file.
/// Also imbeds the path field.
///
/// # Arguments
/// - `deb` The `DebFile` for which will will generate the Pkg struct for.
///
/// # Returns
/// - `Pkg` A struct containing info about the package.
pub fn extract_info(deb: &DebFile) -> Result<Pkg, String> {
    let dir = TempDir::new().unwrap();
    extract_deb_full(&deb, dir.path())?;
    let mut control_file = File::open(dir.path().join("DEBIAN").join("control")).unwrap();
    let mut control_string = String::new();
    std::io::Read::read_to_string(&mut control_file, &mut control_string).unwrap();

    let mut name = String::new();
    let mut version = String::new();
    let mut arch = String::new();
    let mut deps = String::new();
    let mut recommends = String::new();
    let mut suggests = String::new();
    let mut pre_depends = String::new();
    let mut enhances = String::new();
    let mut description = String::new();
    let mut maintainer = String::new();

    let mut block: String = "".to_string();
    for line in control_string.lines() {
        if line.starts_with("Package: ") {
            block = "".to_string();
            // info!("{}", line);
            line[9..].clone_into(&mut name)
        } else if line.starts_with("Version: ") {
            block = "".to_string();
            // info!("{}", line);
            line[9..].clone_into(&mut version)
        } else if line.starts_with("Architecture: ") {
            block = "".to_string();
            // info!("{}", line);
            line[14..].clone_into(&mut arch)
        } else if line.starts_with("Depends: ") {
            block = "".to_string();
            // info!("{}", line);
            line[9..].clone_into(&mut deps)
        } else if line.starts_with("Recommends: ") {
            block = "".to_string();
            // info!("{}", line);
            line[12..].clone_into(&mut recommends)
        } else if line.starts_with("Suggests: ") {
            block = "".to_string();
            // info!("{}", line);
            line[11..].clone_into(&mut suggests)
        } else if line.starts_with("Pre-Depends: ") {
            block = "".to_string();
            // info!("{}", line);
            line[13..].clone_into(&mut pre_depends)
        } else if line.starts_with("Enhances: ") {
            block = "".to_string();
            // info!("{}", line);
            line[11..].clone_into(&mut enhances)
        } else if line.starts_with("Maintainer: ") {
            block = "".to_string();
            // info!("{}", line);
            line[12..].clone_into(&mut maintainer)
        } else if line.starts_with("Homepage: ") {
            block = "".to_string();
            // info!("{}", line);
            line[10..].clone_into(&mut maintainer)
        } else if line.starts_with("Description: ") {
            block = "Description".to_string();
            line[13..].clone_into(&mut description)
        } else if line.starts_with(" ") {
            if block != "".to_string() {
                if block == "Description" {
                    description.push_str(&format!("{}\n", &line[1..]));
                }
            }
        }
    }
    // info!("Description: {}", description);
    Ok(Pkg {
        name,
        version: PackageVersion::parse(&version).unwrap(),
        arch,
        deps,
        recommends,
        suggests,
        pre_depends,
        enhances,
        description,
        maintainer,
        homepage: "".to_string(),
        path: deb.path.to_path_buf(),
    })
}

/// Installs a DebFile into the path specified by root
pub fn install_deb_pkg(d: &DebFile, root: &Path) -> Result<(), String> {
    let info = extract_info(d)?;
    let pkg_dir = root.join(format!(
        "System/Packages/{}---{}",
        info.name,
        info.version.to_string()
    ));
    {
        let dir = TempDir::new().unwrap();
        extract_deb(&d, dir.path())?;
        let out = system::copy_recursive(&dir.path().to_path_buf(), &pkg_dir);
        if let Ok(_) = out {
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
Recommends = '{}'
Suggests = '{}'
Pre-Depends = '{}'
Enhances = '{}'
Maintainer = '{}'
Description = '''{}'''

[Other]
source = 'deb'",
        info.name,
        info.version.to_string(),
        info.arch,
        info.deps,
        info.recommends,
        info.suggests,
        info.pre_depends,
        info.enhances,
        info.maintainer,
        info.description
    );
    // info!("Pkg info:\n{}", info);
    system::touch(&pkg_info_path)?;
    fs::write(&pkg_info_path, info).unwrap();
    // info!("Wrote pkg-info to {}", &pkg_info_path.display());

    Ok(())
}
