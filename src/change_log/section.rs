mod cc_commit;
mod change_log_class;

use std::{collections::HashSet, fmt::Display};

use chrono::NaiveDate;
use semver::Version;

use crate::change_log::{
    section::{cc_commit::ConvCommit, change_log_class::ChangeLogClass},
    tag::Tag,
};

#[derive(Debug, Default, Clone)]
pub(crate) struct Section {
    tag: Option<Tag>,
    title: String,
    version: Option<Version>,
    date: Option<NaiveDate>,
    headings: HashSet<String>,
    description: String,
    yanked: bool,
    // Added for new features.
    added_commits: Vec<ConvCommit>,
    // Fixed for any bug fixes.
    fixed_commits: Vec<ConvCommit>,
    // Changed for changes in existing functionality.
    changed_commits: Vec<ConvCommit>,
    // Security in case of vulnerabilities.
    security_commits: Vec<ConvCommit>,
    // Build changes .
    build_commits: Vec<ConvCommit>,
    // Test commits for changes to testing code
    test_commits: Vec<ConvCommit>,
    // Documentation for updates to code documentation and readme
    documentation_commits: Vec<ConvCommit>,
    // Chores commits
    chore_commits: Vec<ConvCommit>,
    // CI commits
    ci_commits: Vec<ConvCommit>,
    // Deprecated for soon-to-be removed features.
    deprecated_commits: Vec<ConvCommit>,
    // Removed for now removed features.
    removed_commits: Vec<ConvCommit>,
    // Miscellaneous commits not fitting any previous classification.
    misc_commits: Vec<ConvCommit>,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "nothing to see yet!")
    }
}

impl Section {
    pub(crate) fn new(tag: Option<Tag>) -> Self {
        let mut headings = HashSet::new();
        headings.insert(String::from("added"));
        headings.insert(String::from("fixed"));
        headings.insert(String::from("changed"));

        Section {
            tag,
            headings,
            ..Default::default()
        }
    }

    pub(crate) fn add_commit(&mut self, summary: Option<&str>, message: Option<&str>) -> &mut Self {
        let conventional_commit = ConvCommit::new(summary, message);
        let class = ChangeLogClass::new(&conventional_commit.kind_string());

        match class {
            ChangeLogClass::Added => self.added_commits.push(conventional_commit),
            ChangeLogClass::Fixed => self.fixed_commits.push(conventional_commit),
            ChangeLogClass::Changed => self.changed_commits.push(conventional_commit),
            ChangeLogClass::Security => self.security_commits.push(conventional_commit),
            ChangeLogClass::Build => self.build_commits.push(conventional_commit),
            ChangeLogClass::Test => self.test_commits.push(conventional_commit),
            ChangeLogClass::Documentation => self.documentation_commits.push(conventional_commit),
            ChangeLogClass::Chore => self.chore_commits.push(conventional_commit),
            ChangeLogClass::CI => self.ci_commits.push(conventional_commit),
            ChangeLogClass::Deprecated => self.deprecated_commits.push(conventional_commit),
            ChangeLogClass::Removed => self.removed_commits.push(conventional_commit),
            ChangeLogClass::Misc => self.misc_commits.push(conventional_commit),
            ChangeLogClass::Unclassified => self.misc_commits.push(conventional_commit),
        }

        self
    }

    pub(crate) fn report_status(&self) -> String {
        format!(
            "Section: {} contains:
        {} added commits
        {} fixed commits
        {} changed commits
        {} security commits
        {} build commits
        {} test commits
        {} documentation commits
        {} chore commits
        {} ci commits
        {} deprecated commits
        {} removed commits
        {} miscellaneous commits",
            if let Some(tag) = &self.tag {
                tag.to_string()
            } else {
                "Unreleased".to_string()
            },
            self.added_commits.len(),
            self.fixed_commits.len(),
            self.changed_commits.len(),
            self.security_commits.len(),
            self.build_commits.len(),
            self.test_commits.len(),
            self.documentation_commits.len(),
            self.chore_commits.len(),
            self.ci_commits.len(),
            self.deprecated_commits.len(),
            self.removed_commits.len(),
            self.misc_commits.len(),
        )
    }

    pub(crate) fn version(&self) -> Option<String> {
        let vs = self.tag.as_ref()?.version()?.to_string();

        Some(vs)
    }

    pub(crate) fn section_markdown(&self) -> String {
        let header = if let Some(t) = &self.tag {
            let version = if t.version().is_some() {
                t.version().unwrap().to_string()
            } else {
                "Unreleased".to_string()
            };

            let date = if t.date().is_some() {
                t.date().unwrap().format("%Y-%m-%d").to_string()
            } else {
                "".to_string()
            };

            format!("## [{version}] - {date}")
        } else {
            "## [Unreleased]".to_string()
        };

        let added = self.commits_markdown("added", &self.added_commits);
        let fixed = self.commits_markdown("fixed", &self.fixed_commits);
        let changed = self.commits_markdown("changed", &self.changed_commits);
        let security = self.commits_markdown("security", &self.security_commits);
        let build = self.commits_markdown("security", &self.build_commits);
        let test = self.commits_markdown("test", &self.test_commits);
        let documentation = self.commits_markdown("documentation", &self.documentation_commits);
        let chore = self.commits_markdown("chore", &self.chore_commits);
        let ci = self.commits_markdown("ci", &self.ci_commits);
        let deprecated = self.commits_markdown("deprecated", &self.deprecated_commits);
        let removed = self.commits_markdown("removed", &self.removed_commits);
        let misc = self.commits_markdown("misc", &self.misc_commits);

        format!(
            "{header}\n\n{}{}{}{}{}{}{}{}{}{}{}{}",
            added.unwrap_or_default(),
            fixed.unwrap_or_default(),
            changed.unwrap_or_default(),
            security.unwrap_or_default(),
            build.unwrap_or_default(),
            test.unwrap_or_default(),
            documentation.unwrap_or_default(),
            chore.unwrap_or_default(),
            ci.unwrap_or_default(),
            deprecated.unwrap_or_default(),
            removed.unwrap_or_default(),
            misc.unwrap_or_default(),
        )
    }

    fn commits_markdown(&self, heading: &str, commits: &[ConvCommit]) -> Option<String> {
        if !self.headings.contains(heading) | commits.is_empty() {
            None
        } else {
            Some(format!(
                "### {heading}\n\n{}\n",
                commits
                    .iter()
                    .map(|c| format!(" - {}\n", c.title_as_string()))
                    .collect::<String>()
            ))
        }
    }
}
