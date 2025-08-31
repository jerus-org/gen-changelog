//! Classify the kinds of Conventional Commit for display in a changelog
//!
//! The classifications are based on Keep a Changelog with an additional `Misc`
//! classification for unknown classifications and `Unclassified` for when the
//! there is no kind identified. Multiple kinds may be given the same
//! classification for the purpose of the changelog.

use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum ChangeLogClass {
    Added,
    Fixed,
    Changed,
    Security,
    Build,
    Documentation,
    Chore,
    CI,
    Test,
    Deprecated,
    Removed,
    Misc,
    Unclassified,
}

impl ChangeLogClass {
    pub(crate) fn new(kind: &str) -> Self {
        match kind.to_lowercase().as_str() {
            "feat" => ChangeLogClass::Added,
            "fix" => ChangeLogClass::Fixed,
            "refactor" => ChangeLogClass::Changed,
            "security" | "dependency" => ChangeLogClass::Security,
            "build" => ChangeLogClass::Build,
            "doc" | "docs" => ChangeLogClass::Documentation,
            "chore" => ChangeLogClass::Chore,
            "ci" => ChangeLogClass::CI,
            "test" => ChangeLogClass::Test,
            "deprecated" => ChangeLogClass::Deprecated,
            "removed" => ChangeLogClass::Removed,
            "misc" => ChangeLogClass::Misc,
            _ => ChangeLogClass::Unclassified,
        }
    }
}

impl Display for ChangeLogClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeLogClass::Added => write!(f, "Added"),
            ChangeLogClass::Build => write!(f, "Build Changes"),
            ChangeLogClass::CI => write!(f, "CI Changes"),
            ChangeLogClass::Changed => write!(f, "Changed"),
            ChangeLogClass::Fixed => write!(f, "Fixed"),
            ChangeLogClass::Security => write!(f, "Security Changes"),
            ChangeLogClass::Documentation => write!(f, "Documentation Changes"),
            ChangeLogClass::Chore => write!(f, "Chore Changes"),
            ChangeLogClass::Test => write!(f, "Test Changes"),
            ChangeLogClass::Deprecated => write!(f, "Deprecated"),
            ChangeLogClass::Removed => write!(f, "Removed"),
            ChangeLogClass::Misc => write!(f, "Miscellaneous"),
            ChangeLogClass::Unclassified => write!(f, "Unknown"),
        }
    }
}
