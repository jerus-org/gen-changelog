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
/// ```rust
/// use your_crate::DisplaySections;
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
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
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
/// # Examples
///
/// ```rust
/// use your_crate::ReleasePattern;
///
/// // Match tags like "v1.0.0", "v0.2.1"
/// let version_tags = ReleasePattern::Prefix("v".to_string());
///
/// // Match tags like "mypackage-v1.0.0"
/// let package_tags = ReleasePattern::PackagePrefix("mypackage-v".to_string());
/// ```
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
/// ```rust
/// use your_crate::ChangeLogConfig;
///
/// // Load config from default file or use defaults
/// let config = ChangeLogConfig::from_file_or_default()?;
///
/// // Load config from specific file
/// let config = ChangeLogConfig::from_file("custom-config.toml")?;
///
/// // Create default config
/// let config = ChangeLogConfig::default();
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
    /// let config = ChangeLogConfig::from_file_or_default()?;
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
    /// ```rust
    /// let config = ChangeLogConfig::from_file("my-config.toml")?;
    /// let config = ChangeLogConfig::from_file(PathBuf::from("configs/changelog.toml"))?;
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
    /// let config = ChangeLogConfig::default();
    ///
    /// // Save to file
    /// config.save(Some("my-config.toml"))?;
    ///
    /// // Print to stdout
    /// config.save(None)?;
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut config = ChangeLogConfig::default();
    /// let new_group = Group::new_with_name_types_and_publish_flag(
    ///     "Custom",
    ///     &["custom"],
    ///     true
    /// );
    /// config.add_group(new_group);
    /// ```
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
    /// # Examples
    ///
    /// ```rust
    /// let config = ChangeLogConfig::default();
    /// match config.release_pattern() {
    ///     ReleasePattern::Prefix(prefix) => println!("Using prefix: {}", prefix),
    ///     ReleasePattern::PackagePrefix(prefix) => println!("Using package prefix: {}", prefix),
    /// }
    /// ```
    pub fn release_pattern(&self) -> &ReleasePattern {
        &self.release_pattern
    }

    /// Returns a reference to the display sections configuration.
    ///
    /// This determines how many changelog sections will be included in the
    /// generated output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = ChangeLogConfig::default();
    /// match config.display_sections() {
    ///     DisplaySections::All => println!("Displaying all sections"),
    ///     DisplaySections::One => println!("Displaying one section"),
    ///     DisplaySections::Custom(n) => println!("Displaying {} sections", n),
    /// }
    /// ```
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
