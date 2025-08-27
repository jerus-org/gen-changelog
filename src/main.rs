use std::path::PathBuf;

use gen_changelog::ChangeLog;
use git2::Repository;

fn main() {
    let mut logging = get_logging(log::LevelFilter::Debug);
    logging.init();

    let repo_dir = PathBuf::new().join(".");
    let repository = Repository::open(&repo_dir)
        .unwrap_or_else(|_| panic!("unable to open the repository at {}", &repo_dir.display()));
    let mut change_log = ChangeLog::new(&repository).unwrap();

    change_log.build().unwrap().print();
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
