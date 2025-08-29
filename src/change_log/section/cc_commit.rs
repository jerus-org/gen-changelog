//! Convention Commit
//!
//! Classified conventional commit based on analysis of the commit summary only.
//! The analysis of the summary determines if it is a conventional commit and
//! provides segregated emoji, commit kind, scope and breaking flag if it is. If
//! not the summary in its entirety is listed in the title. The commit summary
//! and body are displayed correctly regardless whether or not the commit is a
//! conventional commit.
//!
//! No analysis is done on the body to separate main information and footer
//! information. This is stored and reported as it is in the original commit
//! message.

use lazy_regex::{Lazy, Regex, lazy_regex};

pub static CONVENTIONAL: Lazy<Regex> = lazy_regex!(
    r"^(?P<emoji>.+\s)?(?P<type>[a-z]+)(?:\((?P<scope>.+)\))?(?P<breaking>!)?: (?P<description>.*)$$"
);

#[derive(Debug, Default, Clone)]
pub(crate) struct ConvCommit {
    title: String,
    emoji: Option<String>,
    kind: Option<String>,
    scope: Option<String>,
    breaking: bool,
    body: String,
}

impl ConvCommit {
    pub(crate) fn new(title: Option<&str>, body: Option<&str>) -> Self {
        let mut cc = ConvCommit::default();
        if let Some(t) = title {
            cc = ConvCommit::parse(t);
        }
        if let Some(b) = body {
            cc.body = b.to_string();
        }
        cc
    }

    fn parse(title: &str) -> Self {
        log::trace!("String to parse: `{title}`");

        let cmt_summary = if let Some(captures) = CONVENTIONAL.captures(title) {
            log::trace!("Captures: {captures:#?}");
            let emoji = captures.name("emoji").map(|m| m.as_str().to_string());
            let kind = captures.name("type").map(|m| m.as_str().to_string());
            let scope = captures.name("scope").map(|m| m.as_str().to_string());
            let breaking = captures.name("breaking").is_some();
            let title = captures
                .name("description")
                .map(|m| m.as_str().to_string())
                .unwrap();

            Self {
                title,
                emoji,
                kind,
                scope,
                breaking,
                body: String::new(),
            }
        } else {
            Self {
                title: title.to_string(),
                emoji: None,
                kind: None,
                scope: None,
                breaking: false,
                body: String::new(),
            }
        };

        log::trace!("Parsed title: {cmt_summary:?}");

        cmt_summary
    }

    pub(crate) fn is_conventional(&self) -> bool {
        self.kind.is_some()
    }

    pub(crate) fn kind_string(&self) -> String {
        self.kind.clone().unwrap_or_default()
    }

    pub(crate) fn title_as_string(&self) -> String {
        format!(
            "{}{}{}{}: {}",
            self.emoji.clone().unwrap_or_default(),
            self.kind.clone().unwrap_or_default(),
            self.scope
                .as_ref()
                .map_or("".to_string(), |s| format!("({s})")),
            if self.breaking { "!" } else { "" },
            self.title,
        )
    }
}

impl std::fmt::Display for ConvCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}: {}\n\n{}",
            self.emoji.clone().unwrap_or_default(),
            self.kind.clone().unwrap_or_default(),
            self.scope
                .as_ref()
                .map_or("".to_string(), |s| format!("({s})")),
            if self.breaking { "!" } else { "" },
            self.title,
            self.body
        )
    }
}

//test module
#[cfg(test)]
mod tests {

    use log::LevelFilter;
    use rstest::rstest;

    use super::*;

    fn get_test_logger() {
        let mut builder = env_logger::Builder::new();
        builder.filter(None, LevelFilter::Debug);
        builder.format_timestamp_secs().format_module_path(false);
        let _ = builder.try_init();
    }

    #[test]
    fn test_cmt_summary_parse() {
        let cmt_summary = ConvCommit::new(Some("feat: add new feature"), None);
        assert_eq!(cmt_summary.title, "add new feature");
        assert_eq!(cmt_summary.kind, Some("feat".to_string()));
        assert_eq!(cmt_summary.scope, None);
        assert!(!cmt_summary.breaking);

        let cmt_summary = ConvCommit::new(Some("feat(core): add new feature"), None);
        assert_eq!(cmt_summary.title, "add new feature");
        assert_eq!(cmt_summary.kind, Some("feat".to_string()));
        assert_eq!(cmt_summary.scope, Some("core".to_string()));
        assert!(!cmt_summary.breaking);

        let cmt_summary = ConvCommit::new(Some("feat(core)!: add new feature"), None);
        assert_eq!(cmt_summary.title, "add new feature");
        assert_eq!(cmt_summary.kind, Some("feat".to_string()));
        assert_eq!(cmt_summary.scope, Some("core".to_string()));
        assert!(cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_with_breaking_scope() {
        let cmt_summary = ConvCommit::new(Some("feat(core)!: add new feature"), None);
        assert_eq!(cmt_summary.title, "add new feature");
        assert_eq!(cmt_summary.kind, Some("feat".to_string()));
        assert_eq!(cmt_summary.scope, Some("core".to_string()));
        assert!(cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_with_security_scope() {
        let cmt_summary = ConvCommit::new(Some("fix(security): fix security vulnerability"), None);
        assert_eq!(cmt_summary.title, "fix security vulnerability");
        assert_eq!(cmt_summary.kind, Some("fix".to_string()));
        assert_eq!(cmt_summary.scope, Some("security".to_string()));
        assert!(!cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_with_deprecate_scope() {
        let cmt_summary = ConvCommit::new(Some("chore(deprecate): deprecate old feature"), None);
        assert_eq!(cmt_summary.title, "deprecate old feature");
        assert_eq!(cmt_summary.kind, Some("chore".to_string()));
        assert_eq!(cmt_summary.scope, Some("deprecate".to_string()));
        assert!(!cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_without_scope() {
        let cmt_summary = ConvCommit::new(Some("docs: update documentation"), None);
        assert_eq!(cmt_summary.title, "update documentation");
        assert_eq!(cmt_summary.kind, Some("docs".to_string()));
        assert_eq!(cmt_summary.scope, None);
        assert!(!cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_issue_172() {
        let cmt_summary = ConvCommit::new(
            Some("chore(config.yml): update jerus-org/circleci-toolkit orb version to 0.4.0"),
            None,
        );
        assert_eq!(
            cmt_summary.title,
            "update jerus-org/circleci-toolkit orb version to 0.4.0"
        );
        assert_eq!(cmt_summary.kind, Some("chore".to_string()));
        assert_eq!(cmt_summary.scope, Some("config.yml".to_string()));
        assert!(!cmt_summary.breaking);
    }

    #[rstest]
    #[case("feat: add new feature", "feat")]
    #[case("✨ feat: add new feature", "feat")]
    #[case("feat: add new feature", "feat")]
    #[case("feat: add new feature", "feat")]
    #[case("feat: add new feature", "feat")]
    #[case("✨ feat: add new feature", "feat")]
    #[case("fix: fix an existing feature", "fix")]
    #[case("🐛 fix: fix an existing feature", "fix")]
    #[case("style: fix typo and lint issues", "style")]
    #[case("💄 style: fix typo and lint issues", "style")]
    #[case("test: update tests", "test")]
    #[case("fix(security): Fix security vulnerability", "fix")]
    #[case("chore(deps): Update dependencies", "chore")]
    #[case("🔧 chore(deps): Update dependencies", "chore")]
    #[case("refactor(remove): Remove unused code", "refactor")]
    #[case("♻️ refactor(remove): Remove unused code", "refactor")]
    #[case("docs(deprecate): Deprecate old API", "docs")]
    #[case("📚 docs(deprecate): Deprecate old API", "docs")]
    #[case("ci(other-scope): Update CI configuration", "ci")]
    #[case("👷 ci(other-scope): Update CI configuration", "ci")]
    #[case("test!: Update test cases", "test")]
    #[case::issue_172(
        "chore(config.yml): update jerus-org/circleci-toolkit orb version to 0.4.0",
        "chore"
    )]
    #[case::with_emoji("✨ feat(ci): add optional flag for push failure handling", "feat")]
    fn test_calculate_kind_and_description(#[case] title: &str, #[case] expected_type: &str) {
        get_test_logger();

        let commit = ConvCommit::new(Some(title), None);
        assert_eq!(expected_type, &commit.kind.unwrap());
    }
}
