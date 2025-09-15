use gen_changelog::ChangeLogConfig;

const DEFAULT_CONFIG_FILE: &str = "gen-changelog.toml";

fn main() {
    let config = ChangeLogConfig::default();

    println!("Default ChangeLogConfig serialized to TOML:");
    let _ = config.save(Some(DEFAULT_CONFIG_FILE));
}
