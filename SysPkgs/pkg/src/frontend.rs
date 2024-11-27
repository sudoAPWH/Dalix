use core::panic;
use std::env::consts::ARCH;
use std::{fs, path::Path};

use debian_packaging::package_version::PackageVersion;
use debian_packaging::repository::release::SourcesFileEntry;
use log::{debug, error, info, warn};

use indicatif::{ProgressBar, ProgressStyle};

use crate::backend::DebPkg;
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
                i.to_string(),
                source.url,
                source.dist,
                source.subtype
            ));

            i += 1;
        }
    }

    system::touch(&pkg_cache.join("index"))?;
    let out = fs::write(&pkg_cache.join("index"), &index_str);
    if let Err(e) = out {
        return Err(e.to_string());
    }

    Ok(())
}

pub fn read_package_lists(root: &Path) {
    let pkg_cache = root.join("System").join("Cache").join("Packages");
    let index = root
        .join("System")
        .join("Cache")
        .join("Packages")
        .join("index");
    let index_str = fs::read_to_string(&index).unwrap();

    for line in index_str.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 4 {
            continue;
        }
    }
}

pub fn retrieve_package(pkg: &PackageSelection, root: &Path) -> Result<(), String> {
    let pkg_arch = pkg.arch;
    let pkg_version = pkg.version;
    
    let arch: = system.get_arch();
    if let Some(a) = pkg_arch {
        arch = a;
    }

    // TODO
    
    Ok(())

}
