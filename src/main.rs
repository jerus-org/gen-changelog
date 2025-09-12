use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use gen_changelog::{ChangeLog, ChangeLogConfig, DisplaySections};
use git2::Repository;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    /// The next version number for unreleased changes
    #[arg(short, long)]
    next_version: Option<String>,
    /// The number of level 2 headings to show in the changelog
    #[arg(short, long)]
    sections: Option<u8>,
    /// The number of level 2 headings to show in the changelog
    #[arg(short, long)]
    config_file: Option<String>,
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
struct ConfigCli {
    #[arg(short, long)]
    save: bool,
}

impl ConfigCli {
    fn run(&self) -> Result<(), gen_changelog::Error> {
        let mut config = ChangeLogConfig::default();
        config.set_display_sections(DisplaySections::Custom(3));
        if self.save {
            log::info!("Saving the default changelog configuration.");
            config.save()?;
        }
        Ok(())
    }
}

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

fn run(args: Cli) -> Result<(), gen_changelog::Error> {
    if let Some(cmds) = args.command {
        match cmds {
            Commands::Configuration(config_cli) => config_cli.run()?,
        }
    } else {
        let repo_dir = PathBuf::new().join(".");
        let repository = Repository::open(&repo_dir)
            .unwrap_or_else(|_| panic!("unable to open the repository at {}", &repo_dir.display()));

        let sections = if let Some(n) = args.sections {
            if n == 1 {
                DisplaySections::One
            } else {
                DisplaySections::Custom(n)
            }
        } else {
            DisplaySections::All
        };

        let mut config = if let Some(cfg) = args.config_file {
            ChangeLogConfig::from_file(cfg)?
        } else {
            ChangeLogConfig::from_file_or_default()?
        };
        log::trace!("initial config to build on: {config:?}");

        config.publish_group("Security");
        config.set_display_sections(sections);

        log::trace!("{config:#?}");

        let mut change_log = default_changelog_build(&repository, config);
        change_log.update_unreleased_to_next_version(args.next_version.as_ref());

        let _ = change_log.save();
        println!("{change_log}");
    }
    Ok(())
}

fn default_changelog_build(repository: &Repository, config: ChangeLogConfig) -> ChangeLog {
    let mut change_log_builder = ChangeLog::builder();

    change_log_builder
        .with_config(config)
        .with_repository(repository)
        .unwrap()
        .build()
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
