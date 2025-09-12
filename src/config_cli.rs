use clap::Parser;
use gen_changelog::{ChangeLogConfig, DisplaySections};

#[derive(Parser, Debug)]
pub(crate) struct ConfigCli {
    #[arg(short, long)]
    save: bool,
}

impl ConfigCli {
    pub(crate) fn run(&self) -> Result<(), gen_changelog::Error> {
        let mut config = ChangeLogConfig::default();
        config.set_display_sections(DisplaySections::Custom(3));
        if self.save {
            log::info!("Saving the default changelog configuration.");
            config.save()?;
        }
        Ok(())
    }
}
