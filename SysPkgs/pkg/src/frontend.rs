use core::panic;
use std::env::consts::ARCH;
use std::{fs, path::Path};

use debian_packaging::package_version::PackageVersion;
use debian_packaging::repository::release::SourcesFileEntry;
use log::{debug, error, info, warn};

use indicatif::{ProgressBar, ProgressStyle};

use crate::backend::Pkg;
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
    subtype: String,
}

pub struct PackageSelection {
    name: String,
    version: Option<PackageVersion>,
    arch: Option<String>,
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
                continue;
            }
            sources.push(PackageSource {
                source_type: parts[0].to_string(),
                url: parts[1].to_string(),
                dist: parts[2].to_string(),
                subtype: parts[3].to_string(),
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
    let _ = system::copy_recursive(&pkg_cache, &pkg_cache_bak); // If this fails it just means we don't have a package cache currently.
    system::rm(&pkg_cache)?;
    system::mkdir(&pkg_cache)?;

    let mut index_str: String = String::new();

    let sources_len = sources.len();
    let bar = ProgressBar::new(sources_len as u64);
    bar.set_message("Updating package lists...");

    for source in sources {
        if source.source_type == "deb" {
            let out = system::wget(
                &format!(
                    "{}/dists/{}/{}/binary-{}/Packages.gz",
                    &source.url,
                    &source.dist,
                    source.subtype,
                    system::get_arch()
                ),
                &pkg_cache.join(i.to_string() + ".gz"),
            );
            if let Err(e) = out {
                error!("{}", e);
                system::rm(&pkg_cache)?;
                system::copy_recursive(&pkg_cache_bak, &pkg_cache)?;
                return Err(e);
            }
            bar.inc(1);
            system::gzip_extract(&pkg_cache.join(i.to_string() + ".gz"))?;

            index_str.push_str(&format!(
                "{} {} {} {}\n",
                i, source.url, source.dist, source.subtype
            ));

            i += 1;
        }
    }

    system::touch(&pkg_cache.join("index"))?;
    let out = fs::write(pkg_cache.join("index"), &index_str);
    if let Err(e) = out {
        return Err(e.to_string());
    }

    Ok(())
}

pub fn read_package_lists(root: &Path) -> Result<Vec<Pkg>, String> {
    let mut ret: Vec<Pkg> = vec![];

    let pkg_cache = root.join("System").join("Cache").join("Packages");
    let index = root
        .join("System")
        .join("Cache")
        .join("Packages")
        .join("index");
    let index_str = fs::read_to_string(&index).expect("Failed to read index");

    for line in index_str.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 4 {
            continue;
        }

        let pkg_list = fs::read_to_string(parts[0])
			.expect(format!("Failed to read package list '{}'", parts[0]).as_str());

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
		let mut homepage = String::new();
		let mut path = String::new();

        let mut block: String = "".to_string();
        for line in pkg_list.lines() {
            if line.starts_with("Package: ") {
                block = "".to_string();
                // info!("{}", line);

				// We will also handle the case of the (potential) previous package
				if name != "".to_string() {
					let pkg = Pkg {
						name: name.clone(),
						version: PackageVersion::parse(&version.clone()).unwrap(),
						arch: arch.clone(),
						deps: deps.clone(),
						recommends: recommends.clone(),
						suggests: suggests.clone(),
						pre_depends: pre_depends.clone(),
						enhances: enhances.clone(),
						description: description.clone(),
						maintainer: maintainer.clone(),
						homepage: homepage.clone(),
						path: Path::new(&path).to_path_buf(),
					};
					ret.push(pkg);
				}

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
                line[10..].clone_into(&mut homepage)
            } else if line.starts_with("Description: ") {
                block = "Description".to_string();
				// info!("{}", line);
                line[13..].clone_into(&mut description)
            } else if line.starts_with(" ") {
                if block != "".to_string() {
                    if block == "Description" {
                        description.push_str(&format!("{}\n", &line[1..]));
                    }
                }
            } else if line.starts_with("Filename: ") {
				block = "".to_string();
				// info!("{}", line);
				line[10..].clone_into(&mut path)
			}
        }
    }
    Ok(ret)
}

pub fn retrieve_package(pkg: &PackageSelection, root: &Path) -> Result<(), String> {
    let arch = &pkg.arch;
    let pkg_version = &pkg.version;

    Ok(())
}
