mod cc_commit;
mod change_log_class;

use crate::config::heading_mgmt::HeadingMgmt;

use std::{collections::BTreeMap, fmt::Display};

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
    summary_flag: bool,
    // commits in the section grouped by class
    commits: BTreeMap<String, Vec<ConvCommit>>,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_markdown())
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
            summary_flag: true,
            commits: Default::default(),
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
                log::debug!("{}", self.report_status(false));
            }
            WalkSetup::HeadToRelease(tag) => {
                revwalk.push_head()?;
                let reference = tag.to_string();
                revwalk.hide_ref(&reference)?;
                log::debug!("Walking from the HEAD to the last release `{tag}`",);
                self.get_commits(revwalk, repository);
                log::debug!("{}", self.report_status(false));
            }
            WalkSetup::FromReleaseToRelease(from_tag, to_tag) => {
                revwalk.push(*from_tag.id())?;
                let reference = to_tag.to_string();
                revwalk.hide_ref(&reference)?;
                log::debug!("Walking from the release `{from_tag}` to release `{to_tag}`");
                self.get_commits(revwalk, repository);
                log::debug!("{}", self.report_status(false));
            }
            WalkSetup::ReleaseToStart(tag) => {
                revwalk.push(*tag.id())?;
                log::debug!("Walking from the first release `{tag}` to first commit");
                self.get_commits(revwalk, repository);
                log::debug!("{}", self.report_status(false));
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

        self
    }

    pub(crate) fn report_status(&self, summary: bool) -> String {
        if summary {
            let mut report = String::from("Summary: ");
            let mut comma_flag = false;
            for (h, c) in self.commits.iter() {
                if h == "Unknown" {
                    continue;
                }
                if comma_flag {
                    report.push_str(", ")
                };
                report.push_str(&format!("{}[{}]", h, c.len()));
                comma_flag = true;
            }
            report.push('\n');
            report.push('\n');
            report
        } else {
            let mut report = format!(
                "Section: {} contains:",
                if let Some(tag) = &self.tag {
                    tag.to_string()
                } else {
                    "Unreleased".to_string()
                }
            );

            for (h, c) in self.commits.iter() {
                report.push('\n');
                report.push_str(&format!("  {} commits under {} heading", c.len(), h));
            }
            report
        }
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

        let mut section_string = String::new();
        let mut contains_commits = false;

        if self.summary_flag {
            contains_commits = true;
            section_string.push_str(&header);
            section_string.push('\n');
            section_string.push('\n');
            section_string.push_str(&self.report_status(true));
        }

        for heading in self.headings.values() {
            log::trace!("Heading `{heading}` flag `{contains_commits}` string `{section_string}`");
            if let Some(commits) = self.commits.get(heading) {
                if !contains_commits {
                    contains_commits = true;
                    section_string.push_str(&header);
                    section_string.push('\n');
                    section_string.push('\n');
                }
                let Some(md) = self.commits_markdown(heading, commits) else {
                    continue;
                };

                section_string.push_str(&md);
            }
        }

        if section_string.is_empty() {
            log::warn!("Section is empty");
        } else {
            log::debug!("constructed section markdown: {section_string}");
        }

        section_string
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
