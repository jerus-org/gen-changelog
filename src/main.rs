use std::error::Error;

mod config_cli;
mod generate_cli;

use clap::{Parser, Subcommand};

use crate::{config_cli::ConfigCli, generate_cli::GenerateCli};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate Changelog
    #[clap(name = "generate")]
    Generate(GenerateCli),
    /// Configuration management
    #[clap(name = "config")]
    Configuration(ConfigCli),
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
            Commands::Generate(generate_cli) => generate_cli.run()?,
            Commands::Configuration(config_cli) => config_cli.run()?,
        }
    }
    Ok(())
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
