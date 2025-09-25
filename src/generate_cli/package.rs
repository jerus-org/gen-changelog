use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Cargo {
    workspace: Option<Workspace>,
    package: Option<Package>,
}

impl Cargo {
    fn package(&self) -> Option<Package> {
        self.package.clone()
    }
}

#[derive(Debug, Deserialize)]
struct Workspace {
    members: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Package {
    name: Option<String>,
}

pub(crate) fn get_packages(root: &Path) -> Option<HashMap<String, PathBuf>> {
    let mut packages = HashMap::new();

    if let Some(root_cargo) = insert_package(root, &mut packages) {
        if let Some(workspace) = root_cargo.workspace {
            if let Some(members) = workspace.members {
                for member in members {
                    let pkg_root = root.join(member);
                    let _ = insert_package(&pkg_root, &mut packages);
                }
            }
        }
    }

    if packages.is_empty() {
        None
    } else {
        Some(packages)
    }
}

fn insert_package(root: &Path, map: &mut HashMap<String, PathBuf>) -> Option<Cargo> {
    let mut ret = None;
    let cargo_file = root.join("Cargo.toml");
    let cargo_string = read_to_string(cargo_file).expect("no `Cargo.toml` found");
    if let Ok(cargo) = toml::from_str::<Cargo>(&cargo_string) {
        if let Some(package) = cargo.package() {
            if let Some(name) = package.name {
                map.insert(name, root.to_path_buf());
                ret = Some(cargo);
            }
        }
    }
    ret
}
