use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize the BTreeMap<u8, String> as BTreeMap<String, u8> for TOML
///
/// This inverts the mapping so that TOML can represent it as:
/// ```toml
/// [headings]
/// "Added" = 1
/// "Fixed" = 2
/// "Changed" = 3
/// ```
pub fn serialize<S>(headings: &BTreeMap<u8, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Invert the mapping: BTreeMap<u8, String> -> BTreeMap<String, u8>
    let inverted: BTreeMap<&String, u8> = headings
        .iter()
        .map(|(order, name)| (name, *order))
        .collect();

    inverted.serialize(serializer)
}

/// Deserialize the BTreeMap<String, u8> from TOML as BTreeMap<u8, String>
///
/// This takes TOML like:
/// ```toml
/// [headings]
/// "Added" = 1
/// "Fixed" = 2
/// "Changed" = 3
/// ```
///
/// And converts it to the internal representation BTreeMap<u8, String>
pub fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<u8, String>, D::Error>
where
    D: Deserializer<'de>,
{
    // First deserialize as BTreeMap<String, u8>
    let string_to_u8_map: BTreeMap<String, u8> = BTreeMap::deserialize(deserializer)?;

    // Then invert it to BTreeMap<u8, String>
    let u8_to_string_map: BTreeMap<u8, String> = string_to_u8_map
        .into_iter()
        .map(|(name, order)| (order, name))
        .collect();

    Ok(u8_to_string_map)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Wrapper {
        #[serde(with = "crate::change_log_config::heading_serde")]
        headings: BTreeMap<u8, String>,
    }

    #[test]
    fn test_serialize_headings_roundtrip() {
        let mut map = BTreeMap::new();
        map.insert(1, "Added".to_string());
        map.insert(2, "Fixed".to_string());
        map.insert(3, "Changed".to_string());

        let wrap = Wrapper {
            headings: map.clone(),
        };
        let toml_str = toml::to_string(&wrap).expect("serialize to toml");

        // Ensure TOML contains inverted key/value pairs
        assert!(toml_str.contains("Added = 1"));
        assert!(toml_str.contains("Fixed = 2"));
        assert!(toml_str.contains("Changed = 3"));

        // And roundtrip back to the same structure
        let de: Wrapper = toml::from_str(&toml_str).expect("deserialize from toml");
        assert_eq!(de, wrap);
    }

    #[test]
    fn test_deserialize_from_literal() {
        let toml_str = r#"
            [headings]
            Added = 1
            Fixed = 2
            Changed = 3
        "#;

        let de: Wrapper = toml::from_str(toml_str).expect("deserialize from toml");

        let mut expected = BTreeMap::new();
        expected.insert(1, "Added".to_string());
        expected.insert(2, "Fixed".to_string());
        expected.insert(3, "Changed".to_string());

        assert_eq!(de.headings, expected);
    }

    #[test]
    fn test_empty_headings_roundtrip() {
        let wrap = Wrapper {
            headings: BTreeMap::new(),
        };
        let toml_str = toml::to_string(&wrap).expect("serialize to toml");
        let de: Wrapper = toml::from_str(&toml_str).expect("deserialize from toml");
        assert_eq!(de, wrap);
    }

    #[test]
    fn test_single_heading_roundtrip() {
        let mut map = BTreeMap::new();
        map.insert(1, "Added".to_string());
        let wrap = Wrapper {
            headings: map.clone(),
        };

        let toml_str = toml::to_string(&wrap).expect("serialize to toml");
        assert!(toml_str.contains("Added = 1"));

        let de: Wrapper = toml::from_str(&toml_str).expect("deserialize from toml");
        assert_eq!(de, wrap);
    }
}
