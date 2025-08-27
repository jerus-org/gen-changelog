use std::collections::BTreeMap;

mod group_mgmt;
use group_mgmt::GroupMgnt;

/// Configuration settings for the Change Log
#[derive(Debug)]
pub struct Config {
    /// Which collections of conventional commits should be displayed identified
    /// by the third level heading under which they will appear in the
    /// changelog.
    ///
    /// Default settings are:
    /// - added (to display feat commits)
    /// - fixed (to display fix commits)
    /// - changed (to display refactor commits)
    headings: BTreeMap<u8, String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut headings = BTreeMap::new();
        headings.add_group("added");
        headings.add_group("fixed");
        headings.add_group("changed");

        Self { headings }
    }
}

impl Config {
    /// Return a clone of the current headings config
    pub fn headings(&self) -> BTreeMap<u8, String> {
        self.headings.clone()
    }

    /// Replace the current headings config with new headings config
    pub fn set_headings(&mut self, group: &str) -> &mut Self {
        self.headings.add_group(group);
        self
    }

    /// Add Miscellaneous group to headings
    pub fn add_miscellaneous_heading(&mut self) -> &mut Self {
        self.headings.add_miscellaneous();
        self
    }

    /// Remove miscellaneous group from headings
    pub fn remove_miscellaneous_heading(&mut self) -> &mut Self {
        self.headings.remove_miscellaneous();
        self
    }
}
