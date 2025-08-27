use std::path::PathBuf;

use gen_changelog::ChangeLog;

fn main() {
    let repo_dir = PathBuf::new().join(".");
    let mut change_log = ChangeLog::new(&repo_dir).unwrap();

    change_log.build().unwrap().print();
}
