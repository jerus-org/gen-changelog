use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::Error;

use cargo_toml::Manifest;

/// A RustPackage structure that contains key data from the rust package manifest.
///
/// A RustPackage consists of:
/// - the root of the package source relative to the workspace
/// - a list of the dependencies used by the package in the workspace
///
#[derive(Debug, Clone)]
pub struct RustPackage {
    pub root: PathBuf,
    pub dependencies: Vec<String>,
}

impl RustPackage {
    fn new(root: PathBuf, dependencies: Vec<String>) -> RustPackage {
        RustPackage { root, dependencies }
    }
}

/// A RustPackage structure that contains key data from the rust package manifest.
///
/// A RustPackage consists of:
/// - the root of the package source relative to the workspace
/// - a list of the dependencies used by the package in the workspace
///
/// # Example
///
///
#[derive(Debug, Clone)]
pub struct RustPackages {
    /// BTreeMap of the packages in the workspace by name
    pub packages_by_name: BTreeMap<String, RustPackage>,
}

impl RustPackages {
    /// Create a RustPackages struct gathering the required data from the
    /// packages that are members of the workspace to create a `[RustPackage]`
    /// and collect into a `[BTreeMap]` identified by name.
    pub fn new(root: &Path) -> Result<RustPackages, Error> {
        log::debug!("getting the rust packages in the repository");
        let mut packages = BTreeMap::new();
        log::debug!("Starting from the root `{}`", root.display());

        // Expecting the Cargo.toml in the root to be a workspace
        let ws_toml = root.join("Cargo.toml");
        let ws = Manifest::from_path(ws_toml)?;
        if let Some(workspace) = ws.workspace {
            for member in workspace.members {
                let pkg_root = root.join(member);
                let pkg_toml = pkg_root.join("Cargo.toml");
                let pkg_manifest = Manifest::from_path(pkg_toml)?;

                // Expecting to have a package to process and add to the list of packages
                if let Some(pkg) = pkg_manifest.package {
                    let mut dependencies: Vec<String> =
                        pkg_manifest.dependencies.keys().cloned().collect();
                    dependencies
                        .append(&mut pkg_manifest.dev_dependencies.keys().cloned().collect());
                    dependencies
                        .append(&mut pkg_manifest.build_dependencies.keys().cloned().collect());

                    let rust_package = RustPackage::new(pkg_root, dependencies);

                    packages.insert(pkg.name, rust_package);
                }
            }
        }

        Ok(RustPackages {
            packages_by_name: packages,
        })
    }
}
