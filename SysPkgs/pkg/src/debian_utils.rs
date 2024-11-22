use std::path::Path;
use crate::system;
use std::process::ExitStatus;
use temp_dir::TempDir;

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
pub fn extract_info(deb: &DebFile) {
    let dir = TempDir::new().unwrap();
    extract_deb_full(deb, dir.path());
    let mut control_file = std::fs::File::open(
            dir.path()
            .join("DEBIAN")
            .join("control")
        ).unwrap();
    let mut control_string = String::new();
    std::io::Read::read_to_string(&mut control_file, &mut control_string).unwrap();

}