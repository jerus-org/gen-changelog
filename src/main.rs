use std::path::PathBuf;

use gen_changelog::{ChangeLog, ChangeLogConfig};
use git2::Repository;

fn main() {
    let mut logging = get_logging(log::LevelFilter::Debug);
    logging.init();

    let repo_dir = PathBuf::new().join(".");
    let repository = Repository::open(&repo_dir)
        .unwrap_or_else(|_| panic!("unable to open the repository at {}", &repo_dir.display()));

    let mut config = ChangeLogConfig::default();
    log::trace!("base config to build on: {config:?}");

    config.publish_group("Security");

    let mut change_log_builder = ChangeLog::builder();
    let change_log = change_log_builder
        .with_config(config)
        .with_repository(&repository)
        .unwrap()
        .build();

    let _ = change_log.save();
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
