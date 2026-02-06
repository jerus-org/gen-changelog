use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::Error;

use cargo_toml::Manifest;

use lazy_regex::{Lazy, Regex, lazy_regex};

/// Regular expression pattern for update rust crate commits.
///
/// Captures named groups:
/// - `crate`: The name of the crate updated
static CRATE: Lazy<Regex> = lazy_regex!(r"^.+update rust crate (?P<crate>[\w-]+).+$");

/// A RustPackage structure that contains key data from the rust package manifest.
///
/// A RustPackage consists of:
/// - the root of the package source relative to the workspace
/// - a list of the dependencies used by the package in the workspace
///
#[derive(Debug, Clone, Default)]
pub struct RustPackage {
    pub root: String,
    pub dependencies: Vec<String>,
}

impl RustPackage {
    fn new(root: String, dependencies: Vec<String>) -> RustPackage {
        RustPackage { root, dependencies }
    }

    /// Test to determine if a commit is related to the rust package
    ///
    /// # Returns
    ///
    /// - true or false
    ///
    /// # Inputs
    ///
    /// - subject - the subject line of the commit
    /// - files_in_commit - a list of the files changed in the commit
    ///
    /// # Tests
    ///
    /// ## Update to dependency
    ///
    /// True if the `subject` indicates an update to a crate on which this package depends.
    ///
    /// ## Files in the package
    ///
    /// True if any of the files changed in the commit are located in the package's directory tree.
    pub fn is_related_to_package(&self, subject: &str, files_in_commit: Vec<PathBuf>) -> bool {
        if self.is_update_to_package_dependency(subject) {
            if !log::log_enabled!(log::Level::Debug) {
                log::debug!("Commit is an update to a dependency");
            }
            if !log::log_enabled!(log::Level::Trace) {
                log::trace!("Commit is an update to a dependency ({subject})");
            }
            return true;
        }

        if self.is_commit_to_package_file(files_in_commit) {
            if !log::log_enabled!(log::Level::Debug) {
                log::debug!("File in package directory tree");
            }
            if !log::log_enabled!(log::Level::Trace) {
                log::trace!("File in package directory tree ({})", self.root);
            }
            return true;
        }

        log::trace!(
            "Commit is not related to the {} package",
            self.root
                .split('/')
                .next_back()
                .unwrap_or("***not found***")
        );
        false
    }

    fn is_update_to_package_dependency(&self, subject: &str) -> bool {
        if let Some(caps) = CRATE.captures(subject) {
            self.dependencies.iter().any(|d| *d == caps[1])
        } else {
            false
        }
    }

    fn is_commit_to_package_file(&self, files_in_commit: Vec<PathBuf>) -> bool {
        let filter = &self.root;

        for file in files_in_commit {
            log::debug!("Test `{}`", file.display());

            if file.display().to_string().starts_with(filter) {
                return true;
            }
        }

        false
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
                let pkg_root = root.join(&member);
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

                    let rust_package = RustPackage::new(member, dependencies);

                    packages.insert(pkg.name, rust_package);
                }
            }
        }

        Ok(RustPackages {
            packages_by_name: packages,
        })
    }
}
