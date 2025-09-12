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
const DEFAULT_CONFIG_FILE: &str = "gen-changelog.toml";

/// How many sections to display in the changelog
///
/// ## Options:
/// - All           [display all sections - the default]
/// - One           [display the most recent section - last release or
///   unreleased]
/// - Custom(num)   [a custom number of sections]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum DisplaySections {
    #[default]
    /// Display all available sections
    All,
    /// Display only the most recent section
    One,
    /// Display the lesser of the specified number and all sections
    Custom(u8),
}

/// Pattern to identify a tag as a release tag.
///
/// Examples of valid patterns are:
/// - v0.2.4
/// - gen-changelog-v0.1.9
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ReleasePattern {
    Prefix(String),
    PackagePrefix(String),
}

/// Configuration settings for the Change Log
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct ChangeLogConfig {
    /// Group conventional commits under a heading and set a flag to display the
    /// heading in the changelog
    groups: HashMap<String, Group>,
    /// Group mappings from the convention commit types collected in the group
    /// to the group name.
    groups_mapping: BTreeMap<String, String>,
    /// Headings to display in the changelog
    ///
    /// Default settings are:
    /// - added         [to display feat commits]
    /// - fixed         [to display fix commits]
    /// - changed       [to display refactor commits]
    #[serde(with = "heading_serde")]
    headings: BTreeMap<u8, String>,
    /// How many sections to display in the changelog
    ///
    /// ## Options:
    /// - All           [display all sections - the default]
    /// - One           [display recent section - last release or unreleased]
    /// - Custom(num)   [a custom number of sections]
    display_sections: DisplaySections,
    /// Pattern to identify a tag as a release tag.
    release_pattern: ReleasePattern,
}

impl Default for ChangeLogConfig {
    fn default() -> Self {
        let mut groups = HashMap::new();
        let mut groups_mapping = BTreeMap::new();

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

        let mut headings = BTreeMap::new();
        headings.add_heading("Added");
        headings.add_heading("Fixed");
        headings.add_heading("Changed");

        log::trace!("default headings to publish {headings:?}");

        let release_pattern = ReleasePattern::Prefix(String::from("v"));

        Self {
            groups,
            groups_mapping,
            headings,
            display_sections: DisplaySections::default(),
            release_pattern,
        }
    }
}

impl ChangeLogConfig {
    /// construct a config struct from the default config file if it exists.
    /// Return the default config struct if the file did not exist.
    pub fn from_file_or_default() -> Result<Self, Error> {
        let file = PathBuf::new().join(DEFAULT_CONFIG_FILE);
        if file.exists() && file.is_file() {
            Ok(ChangeLogConfig::from_file(file)?)
        } else {
            Ok(ChangeLogConfig::default())
        }
    }

    /// construct a config struct from the file in the specified path
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, Error> {
        let file = read_to_string(path.into())?;
        Ok(toml::from_str::<ChangeLogConfig>(&file)?)
    }
    /// Returns a reference to the btree storing the ordered list headings to
    /// publish in the change log.
    pub fn headings(&self) -> &BTreeMap<u8, String> {
        &self.headings
    }

    /// Returns a reference to the btree storing the groups.
    pub fn groups_mapping(&self) -> &BTreeMap<String, String> {
        &self.groups_mapping
    }

    /// Save the config file.
    pub fn save(&self) -> Result<(), Error> {
        let toml_string = toml::to_string_pretty(self)?;
        std::fs::write(DEFAULT_CONFIG_FILE, toml_string)?;
        Ok(())
    }
}

impl ChangeLogConfig {
    /// Add to the list of published groups
    pub fn add_commit_groups(&mut self, groups: &[String]) -> &mut Self {
        for g in groups {
            self.publish_group(&g.titlecase());
        }
        self
    }

    /// Set a group to be published in the changelog.
    ///
    /// The flag is updated in the group record and the heading is added to the
    /// next available slot on the headings list.
    pub fn publish_group(&mut self, group_name: &str) -> &mut Self {
        self.groups.set_to_publish(group_name);
        self.headings.add_heading(group_name);
        log::trace!("headings to publish: `{:?}`", self.headings);
        self
    }

    /// Remove from the list of published groups
    pub fn remove_commit_groups(&mut self, groups: &[String]) -> &mut Self {
        for g in groups {
            self.unpublish_group(&g.titlecase());
        }
        self
    }

    /// Set a group not to be published in the changelog.
    ///
    /// The flag is updated in the group record and the heading is added to the
    /// next available slot on the headings list.
    pub fn unpublish_group(&mut self, group_name: &str) -> &mut Self {
        self.groups.unset_to_publish(group_name);
        self.headings.remove_heading(group_name);
        self
    }

    /// Add a group to the group collection.
    ///
    /// The group name is added to the headings in the next available position
    /// if publish flag is set.
    pub fn add_group(&mut self, group: Group) -> &mut Self {
        if group.publish() {
            let name = group.name().to_string();
            self.headings.add_heading(&name);
        }

        self.groups.add_group(group);
        self
    }

    /// Return a reference to release_pattern
    pub fn release_pattern(&self) -> &ReleasePattern {
        &self.release_pattern
    }

    /// Get a reference to the current display sections value
    pub fn display_sections(&self) -> &DisplaySections {
        &self.display_sections
    }

    /// Set a new `display_sections` value
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
