use std::fmt::Display;

use crate::change_log::tag::Tag;

#[derive(Debug, Clone, Default)]
pub(crate) struct SectionHeader(String);

impl From<&Option<Tag>> for SectionHeader {
    fn from(value: &Option<Tag>) -> Self {
        let header = if let Some(t) = value {
            let version = if let Some(version) = t.version() {
                version.to_string()
            } else {
                String::from("Unreleased")
            };

            let date = if let Some(d) = t.date() {
                d.format("%Y-%m-%d").to_string()
            } else {
                String::new()
            };

            format!("## [{version}] - {date}")
        } else {
            "## [Unreleased]".to_string()
        };

        SectionHeader(header)
    }
}

impl Display for SectionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
