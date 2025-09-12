use clap::Parser;
use gen_changelog::ChangeLogConfig;

const DEFAULT_CONFIG_FILE: &str = "gen-changelog.toml";

#[derive(Parser, Debug)]
pub(crate) struct ConfigCli {
    /// Save the configuration to a file
    #[arg(short, long)]
    save: bool,
    /// Name of file for configuration
    #[arg(short, long, default_value = DEFAULT_CONFIG_FILE)]
    file: String,
}

impl ConfigCli {
    pub(crate) fn run(&self) -> Result<(), gen_changelog::Error> {
        let mut config = ChangeLogConfig::default();
        // setting a default number of sections of 3
        config.set_display_sections(Some(3));
        if self.save {
            log::info!("Saving the default changelog configuration.");
            config.save(&self.file)?;
        }
        Ok(())
    }
}
