//! Classify the kinds of Conventional Commit for display in a changelog
//!
//! The classifications are based on Keep a Changelog with an additional `Misc`
//! classification for unknown classifications and `Unclassified` for when the
//! there is no kind identified. Multiple kinds may be given the same
//! classification for the purpose of the changelog.

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
