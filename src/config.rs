use std::collections::{BTreeMap, HashMap};

mod group;
mod group_mgmt;
pub(crate) mod heading_mgmt;

use group::Group;
use group_mgmt::GroupMgmt;
use heading_mgmt::HeadingMgmt;

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

/// How many sections to display in the changelog
///
/// ## Options:
/// - All           [display all sections - the default]
/// - One           [display the most recent section - last release or
///   unreleased]
/// - Custom(num)   [a custom number of sections]
#[derive(Debug, Default)]
pub enum DisplaySections {
    #[default]
    All,
    One,
    Custom(usize),
}

/// Pattern to identify a tag as a release tag.
///
/// Examples of valid patterns are:
/// - v0.2.4
/// - gen-changelog-v0.1.9
#[derive(Debug, Clone)]
pub enum ReleasePattern {
    Prefix(String),
    PackagePrefix(String),
}

/// Configuration settings for the Change Log
#[derive(Debug)]
pub struct Config {
    /// Group conventional commits under a heading and set a flag to display the
    /// heading in the changelog
    groups: HashMap<String, Group>,
    /// Headings to display in the changelog
    ///
    /// Default settings are:
    /// - added         [to display feat commits]
    /// - fixed         [to display fix commits]
    /// - changed       [to display refactor commits]
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

impl Default for Config {
    fn default() -> Self {
        let mut groups = HashMap::new();

        for g in DEFAULT_GROUPS {
            let group = Group::new_with_name_types_and_publish_flag(g.0, g.1, g.2);
            groups.add_group(group);
        }
        log::debug!("default groups {groups:?}");

        let publish_groups: Vec<&Group> = groups
            .iter()
            .filter(|item| item.1.publish())
            .map(|item| item.1)
            .collect();

        log::debug!("{} groups to publish in change log", publish_groups.len());

        let mut headings = BTreeMap::new();
        for group in publish_groups {
            let heading = group.name();
            headings.add_heading(heading);
        }
        log::debug!("default headings to publish {headings:?}");

        let release_pattern = ReleasePattern::Prefix(String::from("v"));

        Self {
            groups,
            headings,
            display_sections: DisplaySections::default(),
            release_pattern,
        }
    }
}

impl Config {
    /// Returns a reference to the btree storing the ordered list headings to
    /// publish in the change log.
    pub fn headings(&self) -> &BTreeMap<u8, String> {
        &self.headings
    }
}

impl Config {
    /// Set a group to be published in the changelog.
    ///
    /// The flag is updated in the group record and the heading is added to the
    /// next available slot on the headings list.
    pub fn publish_group(&mut self, group_name: &str) -> &mut Self {
        self.groups.set_to_publish(group_name);
        self.headings.add_heading(group_name);
        log::debug!("headings to publish: `{:?}`", self.headings);
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
    pub fn set_display_sections(&mut self, value: DisplaySections) -> &mut Self {
        self.display_sections = value;
        self
    }
}
