use clap::Parser;
use gen_changelog::ChangeLogConfig;

#[derive(Parser, Debug)]
pub(crate) struct ConfigCli {
    #[arg(short, long)]
    save: bool,
}

impl ConfigCli {
    pub(crate) fn run(&self) -> Result<(), gen_changelog::Error> {
        let mut config = ChangeLogConfig::default();
        // setting a default number of sections of 3
        config.set_display_sections(Some(3));
        if self.save {
            log::info!("Saving the default changelog configuration.");
            config.save()?;
        }
        Ok(())
    }
}
