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
    let bytes = backup("README.md").expect("unable to create backup copy of README.md");
    println!("Wrote `{bytes}` bytes to backup README.md");

    // remove README.md
    fs::remove_file("README.md").expect("failed to remove README.md");

    // Recreate README.md based on docs data
    let head_file = PathBuf::new().join("docs").join("readme").join("head.md");
    let lib_file = PathBuf::new().join("docs").join("lib.md");
    let main_file = PathBuf::new().join("docs").join("main.md");
    let tail_file = PathBuf::new().join("docs").join("readme").join("tail.md");

    let head = read_to_string(head_file).unwrap_or_default();
    let lib = read_to_string(lib_file).unwrap_or_default();
    let main = read_to_string(main_file).unwrap_or_default();
    let tail = read_to_string(tail_file).unwrap_or_default();

    let readme = format!("{head}{lib}{main}{tail}");
    let buffer = readme.as_bytes();

    fs::write("README.md", buffer).expect("could not write new README.md");
}

fn backup(file: &str) -> Result<u64, std::io::Error> {
    let timestamp = chrono::Utc::now().timestamp();
    let ts = base62::encode(timestamp as u64);

    let backup_file = format!("{file}-{ts}");
    fs::copy(file, backup_file)
}
