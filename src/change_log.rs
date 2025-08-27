mod section;
mod tag;

use std::path::{Path, PathBuf};

use git2::Repository;
use lazy_regex::{Lazy, Regex, lazy_regex};
use section::Section;
use tag::Tag;
use thiserror::Error;

const DEFAULT_HEADER: &str = r##"
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

"##;

const DEFAULT_FOOTER: &str = r##""##;

pub static REMOTE: Lazy<Regex> = lazy_regex!(
    r"^((https://github\.com/)|(git@github.com:))(?P<owner>[a-z\-|A-Z]+)/(?P<repo>[a-z\-_A-Z]+)\.git$$"
);

#[derive(Debug, Error)]
pub enum ChangeLogError {
    /// Error from the git2 crate
    #[error("Git2 says: {0}")]
    Git2Error(#[from] git2::Error),
    // /// Error from the regex crate
    // #[error("Regex says: {0}")]
    // RegexError(#[from] regex::Error),
}

/// ChangeLog main struct
#[derive(Debug)]
pub struct ChangeLog {
    repo_dir: PathBuf,
    owner: String,
    repo: String,
    header: String,
    sections: Vec<Section>,
    footer: String,
}

impl Default for ChangeLog {
    fn default() -> Self {
        ChangeLog {
            repo_dir: PathBuf::new().join("."),
            owner: String::default(),
            repo: String::default(),
            header: DEFAULT_HEADER.to_string(),
            footer: DEFAULT_FOOTER.to_string(),
            sections: Vec::default(),
        }
    }
}

impl ChangeLog {
    /// create new ChangeLog struct
    pub fn new(repo_dir: &Path) -> Result<ChangeLog, ChangeLogError> {
        let (owner, repo) = ChangeLog::get_remote_details(repo_dir)?;

        Ok(ChangeLog {
            repo_dir: PathBuf::new().join(repo_dir),
            owner,
            repo,
            ..Default::default()
        })
    }

    fn get_remote_details(repo_dir: &Path) -> Result<(String, String), ChangeLogError> {
        let repository = Repository::open(repo_dir)?;

        let config = repository.config()?;
        let url = config.get_entry("remote.origin.url")?;
        let Some(haystack) = url.value() else {
            return Ok((String::new(), String::new()));
        };

        let captures = REMOTE.captures(haystack);

        let Some(caps) = captures else {
            return Ok((String::new(), String::new()));
        };

        let owner = caps.name("owner").map_or("", |m| m.as_str()).to_string();
        let repo = caps.name("repo").map_or("", |m| m.as_str()).to_string();

        Ok((owner, repo))
    }

    /// set header
    pub fn set_header(&mut self, value: &str) -> &mut Self {
        self.header = value.to_string();
        self
    }

    /// set footer
    pub fn set_footer(&mut self, value: &str) -> &mut Self {
        self.footer = value.to_string();
        self
    }

    /// Build the sections of the change log
    pub fn build(&mut self) -> Result<&mut Self, ChangeLogError> {
        let repo = Repository::open(&self.repo_dir)?;

        let mut tags = Vec::new();

        repo.tag_foreach(|id, name| {
            let name = String::from_utf8(name.to_vec()).unwrap_or("invalid utf8".to_string());
            let tag = Tag::new(id, name, &repo);
            tags.push(tag);
            true
        })?;

        println!(
            "Tags:\n{}",
            tags.iter()
                .map(|t| t.name().to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );

        let mut revwalk = repo.revwalk()?;

        revwalk.set_sorting(git2::Sort::NONE)?;
        revwalk.push_head()?;
        log::debug!("starting the walk from the HEAD");
        // log::debug!("the reference to walk back to is: `{reference}`");
        // revwalk.hide_ref(reference)?;

        let mut current_section = Section::new(None);

        for oid in revwalk.flatten() {
            if let Some(tag) = tags.iter().find(|t| t.id() == &oid) {
                println!("found the tag: `{tag}`");
                println!("{}", current_section.report_status());
                self.sections.push(current_section);
                current_section = Section::new(Some(tag.clone()));
            };

            let Ok(commit) = repo.find_commit(oid) else {
                continue;
            };

            let Some(summary) = commit.summary() else {
                continue;
            };
            let body = commit.body();
            // println!("Found commit with Summary:\t`{summary}.");
            current_section.add_commit(Some(summary), body);
        }
        println!("{}", current_section.report_status());
        self.sections.push(current_section);

        Ok(self)
    }

    /// Print the change log to standard out
    pub fn print(&self) {
        let report = self
            .sections
            .iter()
            .map(|s| s.section_markdown())
            .collect::<String>();

        let mut footer_vec = Vec::new();

        if !self.owner.is_empty() && !self.repo.is_empty() {
            //     if self.sections.len() == 1 {
            //         footer_vec.push(format!(
            //             "[Unreleased]: https://github.com/{}/{}/commits",
            //             self.owner, self.repo
            //         ));
            //     } else {
            let mut first = true;
            let mut last_version = String::from("");
            println!("Processing `{}` sections", self.sections.len());
            for section in self.sections.iter().rev() {
                if first {
                    if let Some(version) = section.version() {
                        footer_vec.push(format!(
                            "[{version}]: https://github.com/{}/{}/tag/v{version}",
                            self.owner, self.repo,
                        ));
                        first = false;
                        last_version = version;
                    } else {
                        footer_vec.push(format!(
                            "[Unreleased]: https://github.com/{}/{}/commits",
                            self.owner, self.repo
                        ));
                    }
                } else if !first {
                    if let Some(version) = section.version() {
                        footer_vec.push(format!(
                                "[{version}]: https://github.com/{}/{}/compare/v{last_version}...v{version}",
                                self.owner, self.repo,
                            ));
                        last_version = version;
                    } else {
                        footer_vec.push(format!(
                            "[Unreleased]: https://github.com/{}/{}/compare/v{last_version}...HEAD",
                            self.owner, self.repo,
                        ));
                    }
                }
            }
        } else {
            log::warn!("unable to build links as owner and repo not known.");
        }

        footer_vec.push(self.footer.clone());
        let footer = footer_vec.join("\n");
        println!("{}{}{}", self.header, report, footer)
    }
}
