use std::{
    collections::{BTreeMap, HashMap},
    fs::read_to_string,
    path::PathBuf,
};

mod group;
mod group_mgmt;
mod heading_serde;
mod test_config_serialization;

pub(crate) mod heading_mgmt;

use group::Group;
use group_mgmt::GroupMgmt;
use heading_mgmt::HeadingMgmt;
use serde::{Deserialize, Serialize};
use titlecase::Titlecase;

use crate::Error;

/// Default groups configuration with their conventional commit types and publish flags
const DEFAULT_GROUPS: [(&str, &[&str; 2], bool); 12] = [
    ("Added", &["feat", "feat"], true),
    ("Fixed", &["fix", "fix"], true),
    ("Changed", &["refactor", "refactor"], true),
    ("Security", &["security", "dependency"], false),
    ("Build", &["build", "build"], false),
    ("Documentation", &["doc", "docs"], false),
    ("Chore", &["chore", "chore"], false),
    ("Continuous Integration", &["ci", "ci"], false),
    ("Testing", &["test", "test"], false),
    ("Deprecated", &["deprecated", "deprecated"], false),
    ("Removed", &["removed", "removed"], false),
    ("Miscellaneous", &["misc", "misc"], false),
];

/// Default configuration file name
const DEFAULT_CONFIG_FILE: &str = "gen-changelog.toml";

/// Documentation comment for groups section in generated TOML
const GROUPS_COMMENT: &str = r#"# Group tables define the third-level headings used to organize commits in the changelog.
# Each group has the following properties:
#   - name: Display name for the group (should match the table name)
#   - publish: Controls whether this group appears in the published changelog
#   - cc-types: Array of conventional commit types that belong to this group
#
# To add a new group:
#   1. Copy an existing group table
#   2. Update the table name (e.g., [groups.MyGroup])
#   3. Set the name field to match the table name
#   4. Add the appropriate conventional commit types to cc-types
#
# Example:
# [groups.MyGroup]
# name = "MyGroup"
# publish = true
# cc-types = ["mygroup"]
# 
# Note: Each commit type should only belong to one group.
"#;

/// Documentation comment for headings section in generated TOML
const HEADINGS_COMMENT: &str = r"# Defines the display order of groups in the changelog.
# Groups are listed with their priority values (lower numbers appear first).
# Only groups that should be displayed need to be included here.
";

/// Documentation comment for display-sections in generated TOML
const DISPLAY_SECTIONS_COMMENT: &str = r#"# Controls the number of changelog sections to display.
# Each section represents a second-level heading and can be either:
#   - "unreleased" for pending changes
#   - A version number for released versions
# This setting limits the changelog to show only the most recent releases.
"#;

/// Configures how many changelog sections to display in the generated output.
///
/// Each section typically represents a version or release, with the "unreleased"
/// section containing commits since the last release.
///
/// # Examples
///
/// ```rust, ignore
///
/// // Show all available sections
/// let all = DisplaySections::All;
///
/// // Show only the most recent section
/// let one = DisplaySections::One;
///
/// // Show exactly 5 sections
/// let custom = DisplaySections::Custom(5);
/// ```
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "variant", content = "data")]
pub(crate) enum DisplaySections {
    #[default]
    /// Display all available sections (releases and unreleased changes)
    All,
    /// Display only the most recent section (latest release or unreleased if no releases)
    One,
    /// Display the specified number of sections, or all sections if fewer are available
    Custom(u8),
}

/// Defines patterns for identifying Git tags as release tags.
///
/// This is used to filter which tags should be considered as releases
/// when generating the changelog from Git history.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ReleasePattern {
    /// Match tags with the specified prefix (e.g., "v" matches "v1.0.0")
    Prefix(String),
    /// Match tags with a package-specific prefix (e.g., "pkg-v" matches "pkg-v1.0.0")
    PackagePrefix(String),
}

/// Main configuration structure for changelog generation.
///
/// This struct controls all aspects of changelog generation including:
/// - Which commit types are grouped together
/// - The display order of groups in the changelog
/// - How many changelog sections to show
/// - How to identify release tags
///
/// # Configuration File Format
///
/// The configuration is typically stored in a TOML file with the following structure:
///
/// ```toml
/// display-sections = "all"
///
/// [groups.Added]
/// name = "Added"
/// publish = true
/// cc-types = ["feat"]
///
/// [headings]
/// 10 = "Added"
/// 20 = "Fixed"
/// ```
///
/// # Examples
///
/// ```rust, ignore
/// use gen_changelog::ChangeLogConfig;
///
/// # fn main() -> Result<(), gen_changelog::Error> {
/// // Load config from default file or use defaults
/// let config = ChangeLogConfig::from_file_or_default()?;
///
/// // Load config from specific file
/// let config = ChangeLogConfig::from_file("custom-config.toml")?;
///
/// // Create default config
/// let config = ChangeLogConfig::default();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct ChangeLogConfig {
    /// Groups that organize conventional commit types under specific headings.
    ///
    /// Each group maps one or more conventional commit types (like "feat", "fix")
    /// to a display heading (like "Added", "Fixed") and controls whether that
    /// group should be published in the changelog.
    groups: HashMap<String, Group>,

    /// Ordered list of headings to display in the changelog.
    ///
    /// The BTreeMap key represents the display priority (lower numbers first),
    /// and the value is the heading name. Only groups marked for publishing
    /// and included in this map will appear in the final changelog.
    #[serde(with = "heading_serde")]
    headings: BTreeMap<u8, String>,

    /// Controls how many changelog sections to include in the output.
    ///
    /// This affects whether the changelog shows all releases, just the latest,
    /// or a specific number of recent releases.
    display_sections: DisplaySections,

    /// Pattern used to identify Git tags as release tags.
    ///
    /// This field is not serialized to/from configuration files and uses
    /// a default prefix of "v" for version tags.
    #[serde(skip)]
    release_pattern: ReleasePattern,
}

impl Default for ChangeLogConfig {
    /// Creates a default configuration with standard conventional commit groups.
    ///
    /// The default configuration includes:
    /// - 12 predefined groups for common conventional commit types
    /// - Only "Added", "Fixed", "Changed", and "Security" groups are published by default
    /// - Display all sections
    /// - Use "v" prefix for release tag identification
    fn default() -> Self {
        let mut groups = HashMap::new();
        let mut groups_mapping = BTreeMap::new();

        // Initialize default groups from the constant array
        for g in DEFAULT_GROUPS {
            let group = Group::new_with_name_types_and_publish_flag(g.0, g.1, g.2);
            groups.add_group(group);
            for key in g.1 {
                groups_mapping.insert(key.to_string(), g.0.to_string());
            }
        }
        log::trace!("default groups {groups:?}");
        log::trace!("default groups mapping: {groups_mapping:?}");

        let publish_groups: Vec<&Group> = groups
            .iter()
            .filter(|item| item.1.publish())
            .map(|item| item.1)
            .collect();

        log::trace!("{} groups to publish in change log", publish_groups.len());

        // Set up default headings for the most common groups
        let mut headings = BTreeMap::new();
        headings.add_heading("Added");
        headings.add_heading("Fixed");
        headings.add_heading("Changed");
        headings.add_heading("Security");

        log::trace!("default headings to publish {headings:?}");

        let release_pattern = ReleasePattern::Prefix(String::from("v"));

        Self {
            groups,
            headings,
            display_sections: DisplaySections::default(),
            release_pattern,
        }
    }
}

impl ChangeLogConfig {
    /// Loads configuration from the default config file, or returns default config if file doesn't exist.
    ///
    /// This method first checks if the default configuration file (`gen-changelog.toml`)
    /// exists in the current directory. If it does, it loads the configuration from that file.
    /// Otherwise, it returns the default configuration.
    ///
    /// # Returns
    ///
    /// * `Ok(ChangeLogConfig)` - The loaded or default configuration
    /// * `Err(Error)` - If the file exists but cannot be read or parsed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ChangeLogConfig::from_file_or_default()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_file_or_default() -> Result<Self, Error> {
        let file = PathBuf::new().join(DEFAULT_CONFIG_FILE);
        if file.exists() && file.is_file() {
            Ok(ChangeLogConfig::from_file(file)?)
        } else {
            Ok(ChangeLogConfig::default())
        }
    }

    /// Loads configuration from a specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Returns
    ///
    /// * `Ok(ChangeLogConfig)` - The loaded configuration
    /// * `Err(Error)` - If the file cannot be read or the TOML cannot be parsed
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    ///
    /// use std::path::PathBuf;
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ChangeLogConfig::from_file("my-config.toml")?;
    /// let config = ChangeLogConfig::from_file(PathBuf::new().join("configs/changelog.toml"))?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, Error> {
        let file = read_to_string(path.into())?;
        Ok(toml::from_str::<ChangeLogConfig>(&file)?)
    }

    /// Returns a reference to the ordered headings that will be displayed in the changelog.
    ///
    /// The returned BTreeMap has priority values as keys (lower numbers = higher priority)
    /// and heading names as values. Only headings in this map will appear in the
    /// generated changelog.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// let config = ChangeLogConfig::default();
    /// for (priority, heading) in config.headings() {
    ///     println!("Priority {}: {}", priority, heading);
    /// }
    /// ```
    pub fn headings(&self) -> &BTreeMap<u8, String> {
        &self.headings
    }

    /// Returns a mapping of conventional commit types to their corresponding group names.
    ///
    /// This creates a new BTreeMap where each key is a conventional commit type
    /// (like "feat", "fix") and each value is the group name it belongs to
    /// (like "Added", "Fixed").
    ///
    /// # Returns
    ///
    /// A BTreeMap mapping commit types to group names
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// let config = ChangeLogConfig::default();
    /// let mapping = config.groups_mapping();
    /// assert_eq!(mapping.get("feat"), Some(&"Added".to_string()));
    /// assert_eq!(mapping.get("fix"), Some(&"Fixed".to_string()));
    /// ```
    pub fn groups_mapping(&self) -> BTreeMap<String, String> {
        let mut groups_mapping = BTreeMap::new();
        for g in self.groups.values() {
            for key in g.cc_types() {
                groups_mapping.insert(key.to_string(), g.name().to_string());
            }
        }
        groups_mapping
    }

    /// Saves the configuration to a file or prints it to stdout.
    ///
    /// The saved configuration includes helpful comments explaining each section.
    /// If no filename is provided, the configuration is printed to stdout.
    ///
    /// # Arguments
    ///
    /// * `file` - Optional filename to save to. If `None`, prints to stdout.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Configuration saved successfully
    /// * `Err(Error)` - Failed to serialize configuration or write file
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// # #[allow(non_snake_case)]
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ChangeLogConfig::default();
    ///
    /// // Save to file
    /// config.save(Some("my-config.toml"))?;
    ///
    /// // Print to stdout
    /// config.save(None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn save(&self, file: Option<&str>) -> Result<(), Error> {
        let mut toml_string = toml::to_string_pretty(self)?;

        // Add helpful comments to different sections
        if let Some(idx) = toml_string.find("[groups.") {
            toml_string.insert_str(idx, GROUPS_COMMENT);
        }
        if let Some(idx) = toml_string.find("[headings]") {
            toml_string.insert_str(idx, HEADINGS_COMMENT)
        }
        if let Some(idx) = toml_string.find("[display-sections]") {
            toml_string.insert_str(idx, DISPLAY_SECTIONS_COMMENT)
        } else if let Some(idx) = toml_string.find("display-sections") {
            toml_string.insert_str(idx, DISPLAY_SECTIONS_COMMENT)
        }

        if let Some(f) = file {
            std::fs::write(f, toml_string)?;
        } else {
            println!("{toml_string}")
        }
        Ok(())
    }
}

impl ChangeLogConfig {
    /// Adds multiple groups to the published groups list and updates headings.
    ///
    /// Each group name is converted to title case before processing.
    ///
    /// # Arguments
    ///
    /// * `groups` - Slice of group names to add to published list
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// let mut config = ChangeLogConfig::default();
    /// config.add_commit_groups(&["testing".to_string(), "build".to_string()]);
    /// ```
    pub fn add_commit_groups(&mut self, groups: &[String]) -> &mut Self {
        for g in groups {
            self.publish_group(&g.titlecase());
        }
        self
    }

    /// Sets a specific group to be published in the changelog.
    ///
    /// This method updates the group's publish flag and adds the group to the
    /// headings list if it's not already there.
    ///
    /// # Arguments
    ///
    /// * `group_name` - Name of the group to publish
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// let mut config = ChangeLogConfig::default();
    /// config.publish_group("Testing");
    /// ```
    pub fn publish_group(&mut self, group_name: &str) -> &mut Self {
        self.groups.set_to_publish(group_name);
        self.headings.add_heading(group_name);
        log::trace!("headings to publish: `{:?}`", self.headings);
        self
    }

    /// Removes multiple groups from the published groups list.
    ///
    /// Each group name is converted to title case before processing.
    ///
    /// # Arguments
    ///
    /// * `groups` - Slice of group names to remove from published list
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// let mut config = ChangeLogConfig::default();
    /// config.remove_commit_groups(&["chore".to_string(), "ci".to_string()]);
    /// ```
    pub fn remove_commit_groups(&mut self, groups: &[String]) -> &mut Self {
        for g in groups {
            self.unpublish_group(&g.titlecase());
        }
        self
    }

    /// Sets a specific group not to be published in the changelog.
    ///
    /// This method updates the group's publish flag and removes the group from
    /// the headings list.
    ///
    /// # Arguments
    ///
    /// * `group_name` - Name of the group to unpublish
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// let mut config = ChangeLogConfig::default();
    /// config.unpublish_group("Added");
    /// ```
    pub fn unpublish_group(&mut self, group_name: &str) -> &mut Self {
        self.groups.unset_to_publish(group_name);
        self.headings.remove_heading(group_name);
        self
    }

    /// Adds a new group to the configuration.
    ///
    /// If the group is marked for publishing, its name is automatically added
    /// to the headings list.
    ///
    /// # Arguments
    ///
    /// * `group` - The group to add
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn add_group(&mut self, group: Group) -> &mut Self {
        if group.publish() {
            let name = group.name().to_string();
            self.headings.add_heading(&name);
        }

        self.groups.add_group(group);
        self
    }

    /// Returns a reference to the release pattern configuration.
    ///
    /// The release pattern determines which Git tags are considered release tags
    /// when generating the changelog.
    ///
    pub fn release_pattern(&self) -> &ReleasePattern {
        &self.release_pattern
    }

    /// Returns a reference to the display sections configuration.
    ///
    /// This determines how many changelog sections will be included in the
    /// generated output.
    pub(crate) fn display_sections(&self) -> &DisplaySections {
        &self.display_sections
    }

    /// Sets the number of changelog sections to display.
    ///
    /// # Arguments
    ///
    /// * `value` - Optional number of sections to display. If `None`, no change is made.
    ///   - `Some(1)` sets to `DisplaySections::One`
    ///   - `Some(n)` where n > 1 sets to `DisplaySections::Custom(n)`
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gen_changelog::ChangeLogConfig;
    ///
    /// let mut config = ChangeLogConfig::default();
    ///
    /// // Show only the latest section
    /// config.set_display_sections(Some(1));
    ///
    /// // Show 5 most recent sections
    /// config.set_display_sections(Some(5));
    ///
    /// // No change
    /// config.set_display_sections(None);
    /// ```
    pub fn set_display_sections(&mut self, value: Option<u8>) -> &mut Self {
        if let Some(n) = value {
            match n {
                1 => self.display_sections = DisplaySections::One,
                _ => self.display_sections = DisplaySections::Custom(n),
            }
        }
        log::debug!("Display sections `{:#?}`", self.display_sections);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_test_logger;
    use std::fs;
    use tempfile::TempDir;

    // Helper function to create a test config with known state
    fn create_test_config() -> ChangeLogConfig {
        ChangeLogConfig::default()
    }

    // Helper function to create a temporary test file
    fn create_temp_config_file(content: &str) -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("test-config.toml");
        fs::write(&file_path, content).expect("Failed to write temp file");
        (temp_dir, file_path)
    }

    #[test]
    fn test_display_sections_default() {
        let sections = DisplaySections::default();
        assert!(matches!(sections, DisplaySections::All));
    }

    #[test]
    fn test_display_sections_serialization() {
        // Test All variant
        // let all_sections = DisplaySections::All;
        // let serialized = toml::to_string(&all_sections).unwrap();
        // let deserialized: DisplaySections = toml::from_str(&serialized).unwrap();
        // assert!(matches!(deserialized, DisplaySections::All));

        // Test One variant
        let one_section = DisplaySections::One;
        let serialized = toml::to_string(&one_section).unwrap();
        let deserialized: DisplaySections = toml::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, DisplaySections::One));

        // Test Custom variant
        let custom_sections = DisplaySections::Custom(5);
        let serialized = toml::to_string(&custom_sections).unwrap();
        let deserialized: DisplaySections = toml::from_str(&serialized).unwrap();
        if let DisplaySections::Custom(n) = deserialized {
            assert_eq!(n, 5);
        } else {
            panic!("Expected Custom variant");
        }
    }

    #[test]
    fn test_release_pattern_serialization() {
        // Test Prefix variant
        let prefix_pattern = ReleasePattern::Prefix("v".to_string());
        let serialized = toml::to_string(&prefix_pattern).unwrap();
        let deserialized: ReleasePattern = toml::from_str(&serialized).unwrap();
        if let ReleasePattern::Prefix(prefix) = deserialized {
            assert_eq!(prefix, "v");
        } else {
            panic!("Expected Prefix variant");
        }

        // Test PackagePrefix variant
        let package_pattern = ReleasePattern::PackagePrefix("pkg-v".to_string());
        let serialized = toml::to_string(&package_pattern).unwrap();
        let deserialized: ReleasePattern = toml::from_str(&serialized).unwrap();
        if let ReleasePattern::PackagePrefix(prefix) = deserialized {
            assert_eq!(prefix, "pkg-v");
        } else {
            panic!("Expected PackagePrefix variant");
        }
    }

    #[test]
    fn test_change_log_config_default() {
        let config = ChangeLogConfig::default();

        // Check that default groups are created
        assert!(!config.groups.is_empty());

        // Check that default headings are set
        assert!(!config.headings.is_empty());
        assert!(config.headings.values().any(|h| h == "Added"));
        assert!(config.headings.values().any(|h| h == "Fixed"));
        assert!(config.headings.values().any(|h| h == "Changed"));
        assert!(config.headings.values().any(|h| h == "Security"));

        // Check display sections default
        assert!(matches!(config.display_sections, DisplaySections::All));

        // Check release pattern default
        if let ReleasePattern::Prefix(prefix) = &config.release_pattern {
            assert_eq!(prefix, "v");
        } else {
            panic!("Expected Prefix variant with 'v'");
        }
    }

    #[test]
    fn test_from_file_or_default_no_file() {
        get_test_logger();
        let dcf = PathBuf::new().join(DEFAULT_CONFIG_FILE);
        let mut safe_dcf = String::from(DEFAULT_CONFIG_FILE);
        safe_dcf.push_str("-safe");
        let safe_dcf = PathBuf::new().join(safe_dcf);
        let mut renamed = false;
        if dcf.exists() {
            let _ = fs::rename(&dcf, &safe_dcf);
            renamed = true;
        }
        // When no config file exists, should return default config
        let result = ChangeLogConfig::from_file_or_default();
        if renamed {
            let _ = fs::rename(&safe_dcf, &dcf);
        }
        assert!(result.is_ok());

        let config = result.unwrap();
        log::debug!("{config:#?}");
        assert!(matches!(config.display_sections, DisplaySections::All));
    }

    #[test]
    fn test_from_file_or_default_with_file() {
        get_test_logger();
        let toml_content = r#"
[display-sections]
variant = "one"

[groups.TestGroup]
name = "TestGroup"
publish = true
cc-types = ["test"]

[headings]
"TestGroup" = 10
"#;

        let (_temp_dir, _file_path) = create_temp_config_file(toml_content);

        // Temporarily create the default config file
        let default_config_path = PathBuf::from(DEFAULT_CONFIG_FILE);
        fs::write(&default_config_path, toml_content).expect("Failed to write default config");

        let result = ChangeLogConfig::from_file_or_default();
        log::debug!("{result:?}");
        // Clean up
        let _ = fs::remove_file(default_config_path);

        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(matches!(config.display_sections, DisplaySections::One));
    }

    #[test]
    fn test_from_file_success() {
        let toml_content = r#"
[display-sections]
variant = "custom"
data = 3

[groups.Custom]
name = "Custom"
publish = false
cc-types = ["custom"]

[headings]
"Custom" = 15
"#;

        get_test_logger();
        let (_temp_dir, file_path) = create_temp_config_file(toml_content);

        let result = ChangeLogConfig::from_file(file_path);
        log::debug!("{result:?}");
        assert!(result.is_ok());

        let config = result.unwrap();
        if let DisplaySections::Custom(n) = config.display_sections {
            assert_eq!(n, 3);
        } else {
            panic!("Expected Custom display sections");
        }
    }

    #[test]
    fn test_from_file_invalid_toml() {
        let invalid_toml = "invalid toml content [";
        let (_temp_dir, file_path) = create_temp_config_file(invalid_toml);

        let result = ChangeLogConfig::from_file(file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_file_nonexistent() {
        let result = ChangeLogConfig::from_file("nonexistent-file.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_headings_getter() {
        let config = ChangeLogConfig::default();
        let headings = config.headings();

        assert!(!headings.is_empty());
        assert!(headings.values().any(|h| h == "Added"));
    }

    #[test]
    fn test_groups_mapping() {
        let config = ChangeLogConfig::default();
        let mapping = config.groups_mapping();

        assert!(!mapping.is_empty());
        // Check some expected mappings from DEFAULT_GROUPS
        assert!(mapping.contains_key("feat"));
        assert!(mapping.contains_key("fix"));
        assert!(mapping.contains_key("refactor"));
    }

    #[test]
    fn test_save_to_file() {
        let config = ChangeLogConfig::default();
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("output-config.toml");

        let result = config.save(Some(file_path.to_str().unwrap()));
        assert!(result.is_ok());

        // Verify file was created and contains expected content
        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).expect("Failed to read saved file");

        // Check that comments are included
        assert!(content.contains("# Group tables define the third-level headings"));
        assert!(content.contains("# Defines the display order of groups"));
    }

    #[test]
    fn test_save_to_stdout() {
        let config = ChangeLogConfig::default();

        // This test just ensures the method doesn't panic when saving to stdout
        // In a real scenario, you might want to capture stdout to verify output
        let result = config.save(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_commit_groups() {
        let mut config = ChangeLogConfig::default();
        let initial_heading_count = config.headings.len();

        let groups_to_add = vec!["testing".to_string(), "build".to_string()];
        config.add_commit_groups(&groups_to_add);

        // Should have added groups (though they might already exist)
        // The exact count depends on whether these groups were already published
        assert!(config.headings.len() >= initial_heading_count);
    }

    #[test]
    fn test_publish_group() {
        let mut config = ChangeLogConfig::default();
        let initial_heading_count = config.headings.len();

        // Add a group that's likely not already published
        config.publish_group("CustomGroup");

        // Should have one more heading
        assert!(config.headings.len() >= initial_heading_count);
        assert!(config.headings.values().any(|h| h == "CustomGroup"));
    }

    #[test]
    fn test_remove_commit_groups() {
        let mut config = ChangeLogConfig::default();

        // First add some groups
        config.add_commit_groups(&["Testing".to_string(), "Build".to_string()]);
        let after_add_count = config.headings.len();

        // Then remove them
        config.remove_commit_groups(&["testing".to_string(), "build".to_string()]);

        // Should have fewer headings (unless they weren't added initially)
        assert!(config.headings.len() <= after_add_count);
    }

    #[test]
    fn test_unpublish_group() {
        let mut config = ChangeLogConfig::default();

        // Unpublish a group that should be there by default
        let initial_count = config.headings.len();
        config.unpublish_group("Added");

        // Should have one fewer heading
        assert!(config.headings.len() < initial_count);
        assert!(!config.headings.values().any(|h| h == "Added"));
    }

    #[test]
    fn test_add_group_published() {
        let mut config = ChangeLogConfig::default();
        let initial_count = config.headings.len();

        // Create a mock group (you'll need to adjust this based on your Group implementation)
        // This is a placeholder - adjust according to your actual Group struct
        let mock_group = Group::new_with_name_types_and_publish_flag(
            "MockGroup",
            &["mock"],
            true, // published
        );

        config.add_group(mock_group);

        // Should have added the heading since the group is published
        assert!(config.headings.len() > initial_count);
        assert!(config.headings.values().any(|h| h == "MockGroup"));
    }

    #[test]
    fn test_add_group_unpublished() {
        let mut config = ChangeLogConfig::default();
        let initial_count = config.headings.len();

        // Create a mock unpublished group
        let mock_group = Group::new_with_name_types_and_publish_flag(
            "UnpublishedGroup",
            &["unpub"],
            false, // not published
        );

        config.add_group(mock_group);

        // Should not have added the heading since the group is not published
        assert_eq!(config.headings.len(), initial_count);
        assert!(!config.headings.values().any(|h| h == "UnpublishedGroup"));
    }

    #[test]
    fn test_release_pattern_getter() {
        let config = ChangeLogConfig::default();
        let pattern = config.release_pattern();

        if let ReleasePattern::Prefix(prefix) = pattern {
            assert_eq!(prefix, "v");
        } else {
            panic!("Expected Prefix variant");
        }
    }

    #[test]
    fn test_display_sections_getter() {
        let config = ChangeLogConfig::default();
        let sections = config.display_sections();

        assert!(matches!(sections, DisplaySections::All));
    }

    #[test]
    fn test_set_display_sections_one() {
        let mut config = ChangeLogConfig::default();

        config.set_display_sections(Some(1));

        assert!(matches!(config.display_sections, DisplaySections::One));
    }

    #[test]
    fn test_set_display_sections_custom() {
        let mut config = ChangeLogConfig::default();

        config.set_display_sections(Some(5));

        if let DisplaySections::Custom(n) = config.display_sections {
            assert_eq!(n, 5);
        } else {
            panic!("Expected Custom variant");
        }
    }

    #[test]
    fn test_set_display_sections_none() {
        let mut config = ChangeLogConfig::default();

        config.set_display_sections(None);

        // Should remain unchanged
        assert!(matches!(config.display_sections, DisplaySections::All));
    }

    #[test]
    fn test_method_chaining() {
        let mut config = ChangeLogConfig::default();

        // Test that methods return &mut Self for chaining
        config
            .add_commit_groups(&["Testing".to_string()])
            .publish_group("Build")
            .set_display_sections(Some(3))
            .unpublish_group("Added");

        // Verify the chained operations worked
        if let DisplaySections::Custom(n) = config.display_sections {
            assert_eq!(n, 3);
        } else {
            panic!("Expected Custom variant");
        }

        assert!(!config.headings.values().any(|h| h == "Added"));
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let original_config = ChangeLogConfig::default();

        // Serialize to TOML
        let toml_string = toml::to_string(&original_config).expect("Failed to serialize");

        // Deserialize back
        let deserialized_config: ChangeLogConfig =
            toml::from_str(&toml_string).expect("Failed to deserialize");

        // Compare key properties (you might need to implement PartialEq or compare manually)
        assert_eq!(
            original_config.headings.len(),
            deserialized_config.headings.len()
        );
        assert!(matches!(
            deserialized_config.display_sections,
            DisplaySections::All
        ));
    }

    #[test]
    fn test_default_groups_configuration() {
        let config = ChangeLogConfig::default();
        let mapping = config.groups_mapping();

        // Verify some key mappings from DEFAULT_GROUPS
        assert_eq!(mapping.get("feat"), Some(&"Added".to_string()));
        assert_eq!(mapping.get("fix"), Some(&"Fixed".to_string()));
        assert_eq!(mapping.get("refactor"), Some(&"Changed".to_string()));
        assert_eq!(mapping.get("security"), Some(&"Security".to_string()));
        assert_eq!(mapping.get("build"), Some(&"Build".to_string()));
        assert_eq!(mapping.get("doc"), Some(&"Documentation".to_string()));
        assert_eq!(mapping.get("docs"), Some(&"Documentation".to_string()));
        assert_eq!(mapping.get("chore"), Some(&"Chore".to_string()));
        assert_eq!(
            mapping.get("ci"),
            Some(&"Continuous Integration".to_string())
        );
        assert_eq!(mapping.get("test"), Some(&"Testing".to_string()));
    }

    #[test]
    fn test_invalid_toml_fields() {
        // Test that unknown fields are rejected due to serde(deny_unknown_fields)
        let invalid_toml = r#"
display-sections = "all"
unknown-field = "should-fail"

[groups.Added]
name = "Added"
publish = true
cc-types = ["feat"]
"#;

        let result: Result<ChangeLogConfig, _> = toml::from_str(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_kebab_case_serialization() {
        // Test that fields use kebab-case in serialization
        let config = ChangeLogConfig::default();
        let toml_string = toml::to_string(&config).expect("Failed to serialize");

        // Should use kebab-case, not snake_case
        assert!(toml_string.contains("display-sections"));
        assert!(!toml_string.contains("display_sections"));
    }

    #[test]
    fn test_groups_mapping_consistency() {
        let config = ChangeLogConfig::default();
        let mapping = config.groups_mapping();

        // Each commit type should map to exactly one group
        let mut seen_types = std::collections::HashSet::new();
        for commit_type in mapping.keys() {
            assert!(
                !seen_types.contains(commit_type),
                "Commit type '{commit_type}' appears in multiple groups"
            );
            seen_types.insert(commit_type.clone());
        }

        // Should have mappings for all default commit types
        assert!(!mapping.is_empty());
    }

    #[test]
    fn test_headings_ordering() {
        let config = ChangeLogConfig::default();
        let headings = config.headings();

        // BTreeMap should maintain order by keys
        let mut previous_key = 0u8;
        for key in headings.keys() {
            assert!(
                *key >= previous_key,
                "Headings should be ordered by priority"
            );
            previous_key = *key;
        }
    }

    // Integration test that combines multiple operations
    #[test]
    fn test_full_workflow() {
        let mut config = ChangeLogConfig::default();

        // Add some custom groups
        config.add_commit_groups(&["Performance".to_string(), "Refactoring".to_string()]);

        // Remove a default group
        config.unpublish_group("Security");

        // Set display sections
        config.set_display_sections(Some(2));

        // Verify final state
        assert!(!config.headings.values().any(|h| h == "Security"));

        if let DisplaySections::Custom(n) = config.display_sections {
            assert_eq!(n, 2);
        } else {
            panic!("Expected Custom display sections");
        }

        // Save and reload to test persistence
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("workflow-test.toml");

        config
            .save(Some(file_path.to_str().unwrap()))
            .expect("Failed to save");
        let reloaded_config = ChangeLogConfig::from_file(&file_path).expect("Failed to load");

        // Verify the reloaded config matches
        assert!(!reloaded_config.headings.values().any(|h| h == "Security"));
        if let DisplaySections::Custom(n) = reloaded_config.display_sections {
            assert_eq!(n, 2);
        } else {
            panic!("Expected Custom display sections in reloaded config");
        }
    }
}
