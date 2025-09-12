use std::path::PathBuf;

use clap::Parser;
use gen_changelog::{ChangeLog, ChangeLogConfig};
use git2::Repository;

#[derive(Parser, Debug)]
pub(crate) struct GenerateCli {
    /// The next version number for unreleased changes
    #[arg(short, long)]
    next_version: Option<String>,
    /// The number of level 2 headings to show in the changelog
    #[arg(short, long)]
    sections: Option<u8>,
    /// The number of level 2 headings to show in the changelog
    #[arg(short, long)]
    config_file: Option<String>,
    /// Path to the repository
    #[arg(short, long, default_value = ".")]
    repo_dir: String,
}

impl GenerateCli {
    pub(crate) fn run(&self) -> Result<(), gen_changelog::Error> {
        log::debug!("Arguments to apply: {self:#?}");
        let repo_dir = PathBuf::new().join(&self.repo_dir);
        let repository = Repository::open(&repo_dir)
            .unwrap_or_else(|_| panic!("unable to open the repository at {}", &repo_dir.display()));

        let mut config = if let Some(cfg) = &self.config_file {
            ChangeLogConfig::from_file(cfg)?
        } else {
            ChangeLogConfig::from_file_or_default()?
        };
        log::debug!("initial config to build on: {config:?}");

        config.publish_group("Security");
        config.set_display_sections(self.sections);

        log::debug!("{config:#?}");

        let mut change_log_builder = ChangeLog::builder();
        let change_log = change_log_builder
            .with_config(config)
            .with_repository(&repository)
            .unwrap()
            .update_unreleased_to_next_version(self.next_version.as_ref())
            .build();

        let _ = change_log.save();
        println!("{change_log}");
        Ok(())
    }
}
