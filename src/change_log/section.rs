mod cc_commit;

use crate::config::heading_mgmt::HeadingMgmt;

use std::{collections::BTreeMap, fmt::Display};

use chrono::NaiveDate;
use git2::{Repository, Revwalk};
use semver::Version;

use crate::change_log::{ChangeLogError, section::cc_commit::ConvCommit, tag::Tag};

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
    groups_mapping: BTreeMap<String, String>,
    // commits in the section grouped by class
    commits: BTreeMap<String, Vec<ConvCommit>>,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_markdown())
    }
}

impl Section {
    pub(crate) fn new(
        tag: Option<Tag>,
        headings: &BTreeMap<u8, String>,
        group_mapping: &BTreeMap<String, String>,
    ) -> Self {
        log::debug!("Section headings to publish: {headings:?}");

        Section {
            tag,
            headings: headings.to_owned(),
            title: Default::default(),
            version: Default::default(),
            date: Default::default(),
            description: Default::default(),
            yanked: Default::default(),
            summary_flag: true,
            groups_mapping: group_mapping.to_owned(),
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
        if let Some(k) = conventional_commit.kind() {
            let group = match k.to_lowercase().as_str() {
                "chore" => {
                    if let Some(s) = conventional_commit.scope() {
                        if s.as_str() == "deps" {
                            "Security".to_string()
                        } else {
                            "Chore".to_string()
                        }
                    } else {
                        "Chore".to_string()
                    }
                }
                _ => {
                    if let Some(g) = self.groups_mapping.get(&k) {
                        g.to_string()
                    } else {
                        "Unknown".to_string()
                    }
                }
            };

            self.add_commit_to_hashmap(&group, conventional_commit.clone());
        }

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
                }
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

    pub(crate) fn get_section_header(&self) -> String {
        if let Some(t) = &self.tag {
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
        }
    }
    pub(crate) fn section_markdown(&self) -> String {
        let header = self.get_section_header();
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
