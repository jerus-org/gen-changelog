use gen_changelog::ChangeLogConfig;

fn main() {
    let config = ChangeLogConfig::default();

    println!("Default ChangeLogConfig serialized to TOML:");
    let _ = config.save();
}
