use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use gen_changelog::{ChangeLog, ChangeLogConfig, ChangeLogError};
use git2::Repository;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    /// The next version number for unreleased changes
    #[arg(short, long)]
    next_version: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Configuration management
    #[clap(name = "config")]
    Configuration(ConfigCli),
}

#[derive(Parser, Debug)]
struct ConfigCli {}

fn main() {
    let args = Cli::parse();

    let mut logging = get_logging(args.logging.log_level_filter());
    logging.init();

    match run(args) {
        Ok(_) => {}
        Err(e) => {
            if let Some(src) = e.source() {
                log::error!("{e}: {src}");
                eprintln!("{e}: {src}");
            } else {
                log::error!("{e}");
                eprintln!("{e}");
            }
            std::process::exit(101);
        }
    }
}

fn run(_args: Cli) -> Result<(), ChangeLogError> {
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

    // let _ = change_log.save();
    println!("{change_log}");
    Ok(())
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
