#[cfg(test)]
mod integration_tests {
    use crate::change_log_config::ChangeLogConfig;
    use std::collections::BTreeMap;

    #[test]
    fn test_changelog_config_serialization_with_headings() {
        let mut config = ChangeLogConfig::default();
        
        // Clear and set specific headings for predictable testing
        config.headings.clear();
        config.headings.insert(1, "Added".to_string());
        config.headings.insert(2, "Fixed".to_string());
        config.headings.insert(3, "Changed".to_string());

        // Serialize to TOML
        let toml_str = toml::to_string_pretty(&config).expect("serialize config to toml");
        
        // Check that headings are serialized in the inverted format
        assert!(toml_str.contains("[headings]"));
        assert!(toml_str.contains("Added = 1"));
        assert!(toml_str.contains("Fixed = 2"));
        assert!(toml_str.contains("Changed = 3"));

        // Deserialize back and verify round trip
        let deserialized: ChangeLogConfig = toml::from_str(&toml_str)
            .expect("deserialize config from toml");

        assert_eq!(config.headings, deserialized.headings);
    }

    #[test]
    fn test_changelog_config_deserialize_from_toml_with_headings() {
        let toml_str = r#"
            display-sections = "all"

            [headings]
            "Added" = 1
            "Fixed" = 2
            "Custom" = 5

            [groups-mapping]
            feat = "Added"
            fix = "Fixed"

            [release-pattern]
            prefix = "v"
        "#;

        let config: ChangeLogConfig = toml::from_str(toml_str)
            .expect("deserialize config from toml");

        let mut expected_headings = BTreeMap::new();
        expected_headings.insert(1, "Added".to_string());
        expected_headings.insert(2, "Fixed".to_string());
        expected_headings.insert(5, "Custom".to_string());

        assert_eq!(config.headings, expected_headings);
        assert_eq!(config.groups_mapping.get("feat"), Some(&"Added".to_string()));
        assert_eq!(config.groups_mapping.get("fix"), Some(&"Fixed".to_string()));
    }
}
