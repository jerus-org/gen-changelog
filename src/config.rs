use std::collections::BTreeMap;

mod group;
// mod group_mgmt;

use group::Group;
// use group_mgmt::GroupMgmt;

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
    /// Group conventional commits under a heading and set a flag to display the heading in the changelog
    ///
    /// Default settings are:
    /// - added         [to display feat commits]
    /// - fixed         [to display fix commits]
    /// - changed       [to display refactor commits]
    groups: BTreeMap<u8, Group>,
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
        let mut groups = BTreeMap::new();
        let added_group = Group::builder();
        let mut added_group = added_group.set_name("added");
        added_group.allow_publication();
        let added_group = added_group.insert_cc_type("fixed").build();
        groups.insert(1, added_group);
        // groups.add_group(added_group);

        // headings.add_group("added");
        // headings.add_group("fixed");
        // headings.add_group("changed");

        let release_pattern = ReleasePattern::Prefix(String::from("v"));

        Self {
            groups,
            display_sections: DisplaySections::default(),
            release_pattern,
        }
    }
}

impl Config {
    /// Return a clone of the current groups config
    pub fn groups(&self) -> BTreeMap<u8, Group> {
        self.groups.clone()
    }

    // /// Replace the current headings config with new headings config
    // pub fn set_groups(&mut self, group: &Group) -> &mut Self {
    //     self.groups.set_group(group);
    //     self
    // }

    // /// Add Miscellaneous group to headings
    // pub fn add_miscellaneous_heading(&mut self) -> &mut Self {
    //     self.headings.add_miscellaneous();
    //     self
    // }

    // /// Remove miscellaneous group from headings
    // pub fn remove_miscellaneous_heading(&mut self) -> &mut Self {
    //     self.headings.remove_miscellaneous();
    //     self
    // }

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
