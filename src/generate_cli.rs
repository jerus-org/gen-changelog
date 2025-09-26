use std::path::PathBuf;

mod package;

use clap::Parser;
use gen_changelog::{ChangeLog, ChangeLogConfig, Error};
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
    /// display summary of commits
    #[arg(short, long)]
    display_summaries: bool,
    /// add commit groups
    #[arg(long)]
    add_groups: Vec<String>,
    /// remove commit groups
    #[arg(long)]
    remove_groups: Vec<String>,
    /// generate the change log for a specific package
    #[arg(short, long)]
    package: Option<String>,
}

impl GenerateCli {
    pub(crate) fn run(&self) -> Result<(), Error> {
        log::debug!("Arguments to apply: {self:#?}");
        let repo_dir = PathBuf::new().join(&self.repo_dir);
        log::debug!("{}", repo_dir.display());
        let repository = Repository::open(&repo_dir)
            .unwrap_or_else(|_| panic!("unable to open the repository at {}", &repo_dir.display()));

        let packages = package::get_packages(&repo_dir)?;
        log::debug!("{packages:?}");
        let pkg_root = if let Some(p) = &self.package {
            packages
                .get(p)
                .unwrap_or(&repo_dir.to_path_buf())
                .to_path_buf()
        } else {
            let root = &repo_dir.to_path_buf();
            root.to_path_buf()
        };

        let config = self.make_config()?;

        let mut change_log_builder = ChangeLog::builder();
        let change_log = change_log_builder
            .with_config(config)
            .with_summary_flag(self.display_summaries)
            .with_package_root(&pkg_root)
            .walk_repository(&repository)
            .unwrap()
            .update_unreleased_to_next_version(self.next_version.as_ref())
            .build();

        let _ = change_log.save();
        // println!("{change_log}");
        Ok(())
    }

    fn make_config(&self) -> Result<ChangeLogConfig, gen_changelog::Error> {
        let mut config = if let Some(cfg) = &self.config_file {
            ChangeLogConfig::from_file(cfg)?
        } else {
            ChangeLogConfig::from_file_or_default()?
        };
        log::debug!("initial config to build on: {config:?}");

        config.publish_group("Security");
        config.set_display_sections(self.sections);
        config.add_commit_groups(&self.add_groups);
        config.remove_commit_groups(&self.remove_groups);

        log::debug!("{config:#?}");
        Ok(config)
    }
}
