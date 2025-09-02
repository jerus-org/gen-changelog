mod cc_commit;
mod change_log_class;

use crate::config::heading_mgmt::HeadingMgmt;

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
};

use chrono::NaiveDate;
use git2::{Repository, Revwalk};
use semver::Version;

use crate::change_log::{
    ChangeLogError,
    section::{cc_commit::ConvCommit, change_log_class::ChangeLogClass},
    tag::Tag,
};

pub(crate) enum WalkSetup<'a> {
    NoReleases,
    HeadToRelease(&'a Tag),
    FromReleaseToRelease(&'a Tag, &'a Tag),
    ReleaseToStart(&'a Tag),
}

#[derive(Debug, Clone)]
pub(crate) struct Section {
    tag: Option<Tag>,
    title: String,
    version: Option<Version>,
    date: Option<NaiveDate>,
    headings: BTreeMap<u8, String>,
    description: String,
    yanked: bool,
    // commits in the section grouped by class
    commits: HashMap<String, Vec<ConvCommit>>,
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

        let added = self.commits_markdown("Added", &self.added_commits);
        let fixed = self.commits_markdown("Fixed", &self.fixed_commits);
        let changed = self.commits_markdown("Changed", &self.changed_commits);
        let security = self.commits_markdown("Security", &self.security_commits);
        let build = self.commits_markdown("Security", &self.build_commits);
        let test = self.commits_markdown("Test", &self.test_commits);
        let documentation = self.commits_markdown("Documentation", &self.documentation_commits);
        let chore = self.commits_markdown("Chore", &self.chore_commits);
        let ci = self.commits_markdown("Continuous Integration", &self.ci_commits);
        let deprecated = self.commits_markdown("Deprecated", &self.deprecated_commits);
        let removed = self.commits_markdown("Removed", &self.removed_commits);
        let misc = self.commits_markdown("Miscellaneous", &self.misc_commits);

        writeln!(
            f,
            "\n{header}\n\n{}{}{}{}{}{}{}{}{}{}{}{}",
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
}

impl Section {
    pub(crate) fn new(tag: Option<Tag>, headings: BTreeMap<u8, String>) -> Self {
        log::debug!("Section headings to publish: {headings:?}");

        Section {
            tag,
            headings,
            title: Default::default(),
            version: Default::default(),
            date: Default::default(),
            description: Default::default(),
            yanked: Default::default(),
            commits: Default::default(),
            added_commits: Default::default(),
            fixed_commits: Default::default(),
            changed_commits: Default::default(),
            security_commits: Default::default(),
            build_commits: Default::default(),
            test_commits: Default::default(),
            documentation_commits: Default::default(),
            chore_commits: Default::default(),
            ci_commits: Default::default(),
            deprecated_commits: Default::default(),
            removed_commits: Default::default(),
            misc_commits: Default::default(),
        }
    }

    pub(crate) fn walk_repository(
        &mut self,
        setup: WalkSetup,
        repository: &Repository,
        revwalk: &mut Revwalk,
    ) -> Result<&mut Self, ChangeLogError> {
        match setup {
            WalkSetup::NoReleases => {
                revwalk.push_head()?;
                log::debug!("Walking from the HEAD to the first commit");
                self.get_commits(revwalk, repository);
                log::debug!("{}", self.report_status());
            }
            WalkSetup::HeadToRelease(tag) => {
                revwalk.push_head()?;
                let reference = tag.to_string();
                revwalk.hide_ref(&reference)?;
                log::debug!("Walking from the HEAD to the last release `{tag}`",);
                self.get_commits(revwalk, repository);
                log::debug!("{}", self.report_status());
            }
            WalkSetup::FromReleaseToRelease(from_tag, to_tag) => {
                revwalk.push(*from_tag.id())?;
                let reference = to_tag.to_string();
                revwalk.hide_ref(&reference)?;
                log::debug!("Walking from the release `{from_tag}` to release `{to_tag}`");
                self.get_commits(revwalk, repository);
                log::debug!("{}", self.report_status());
            }
            WalkSetup::ReleaseToStart(tag) => {
                revwalk.push(*tag.id())?;
                log::debug!("Walking from the first release `{tag}` to first commit");
                self.get_commits(revwalk, repository);
                log::debug!("{}", self.report_status());
            }
        }

        Ok(self)
    }

    fn get_commits(&mut self, revwalk: &mut Revwalk, repository: &Repository) -> &mut Self {
        for oid in revwalk.flatten() {
            let Ok(commit) = repository.find_commit(oid) else {
                continue;
            };

            let summary = commit.summary();
            let body = commit.body();
            if summary.is_some() {
                log::trace!("Found commit with Summary:\t`{}.", summary.unwrap());
                self.add_commit(summary, body);
            }
        }

        self
    }

    fn add_commit_to_hashmap(&mut self, class: &str, commit: ConvCommit) {
        let key = class.to_string();
        let mut new_value = if let Some(v) = self.commits.get(class) {
            v.clone()
        } else {
            Vec::new()
        };

        new_value.push(commit);
        self.commits.insert(key, new_value);
    }

    pub(crate) fn add_commit(&mut self, summary: Option<&str>, message: Option<&str>) -> &mut Self {
        let conventional_commit = ConvCommit::new(summary, message);
        let class = ChangeLogClass::new(&conventional_commit.kind_string());

        self.add_commit_to_hashmap(&class.to_string(), conventional_commit.clone());

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
        let mut report = format!(
            "Section: {} contains:",
            if let Some(tag) = &self.tag {
                tag.to_string()
            } else {
                "Unreleased".to_string()
            }
        );

        if !self.added_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!("  {} added commits", self.added_commits.len()));
        }

        if !self.fixed_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!("  {} fixed commits", self.fixed_commits.len()));
        }

        if !self.changed_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!("  {} changed commits", self.changed_commits.len()));
        }

        if !self.security_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!(
                "  {} security commits",
                self.security_commits.len()
            ));
        }

        if !self.build_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!("  {} build commits", self.build_commits.len()));
        }

        if !self.test_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!("  {} test commits", self.test_commits.len()));
        }

        if !self.documentation_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!(
                "  {} documentation commits",
                self.documentation_commits.len()
            ));
        }

        if !self.chore_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!("  {} chore commits", self.chore_commits.len()));
        }

        if !self.ci_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!("  {} ci commits", self.ci_commits.len()));
        }

        if !self.deprecated_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!(
                "  {} deprecated commits",
                self.deprecated_commits.len()
            ));
        }

        if !self.removed_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!("  {} removed commits", self.removed_commits.len()));
        }

        if !self.misc_commits.is_empty() {
            report.push('\n');
            report.push_str(&format!(
                "  {} miscellaneous commits",
                self.misc_commits.len()
            ));
        }

        report
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
