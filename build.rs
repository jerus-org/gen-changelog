use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

fn main() {
    make_readme();
}

// Assemble the readfile from three components:
// 1. readme-head from docs/readme/head.md
// 2. library-doc from docs/lib.md
// 3. readme-tail from docs/readme/tail.md
fn make_readme() {
    // Backup the README.md
    fs::copy("README.md", "README.old").expect("unable to create backup copy of README.md");

    // remove README.md
    fs::remove_file("README.md").expect("failed to remove README.md");

    // Recreate README.md based on docs data
    let head_file = PathBuf::new().join("docs").join("readme").join("head.md");
    let lib_file = PathBuf::new().join("docs").join("lib.md");
    let tail_file = PathBuf::new().join("docs").join("readme").join("tail.md");

    let head = read_to_string(head_file).expect("could not read readme head");
    let lib = read_to_string(lib_file).expect("could not read library docs");
    let tail = read_to_string(tail_file).expect("cound not read readme tail");

    let readme = format!("{head}{lib}{tail}");
    let buffer = readme.as_bytes();

    fs::write("README.md", buffer).expect("could not write new README.md");
}
