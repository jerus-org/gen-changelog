mod header;
mod link;
mod section;
mod tag;

use std::fmt::Debug;

use git2::Repository;
use header::Header;
use lazy_regex::{Lazy, Regex, lazy_regex};
use link::Link;
use section::Section;
use tag::Tag;
use thiserror::Error;

use crate::Config;

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
pub struct ChangeLog<'a> {
    repository: &'a Repository,
    owner: String,
    repo: String,
    header: Header,
    sections: Vec<Section>,
    links: Vec<Link>,
    config: Config,
}

impl<'a> Debug for ChangeLog<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChangeLog")
            .field("owner", &self.owner)
            .field("repo", &self.repo)
            .field("header", &self.header)
            .field("sections", &self.sections)
            .field("links", &self.links)
            .finish()
    }
}

impl<'a> ChangeLog<'a> {
    /// create new ChangeLog struct
    pub fn new(repository: &Repository) -> Result<ChangeLog<'_>, ChangeLogError> {
        let (owner, repo) = ChangeLog::get_remote_details(repository)?;

        Ok(ChangeLog {
            repository,
            owner,
            repo,
            header: Header::default(),
            links: Vec::new(),
            sections: Vec::default(),
            config: Config::default(),
        })
    }

    /// create new ChangeLog struct with config
    pub fn new_with_config(
        repository: &Repository,
        config: Config,
    ) -> Result<ChangeLog<'_>, ChangeLogError> {
        let (owner, repo) = ChangeLog::get_remote_details(repository)?;

        Ok(ChangeLog {
            repository,
            owner,
            repo,
            header: Header::default(),
            links: Vec::new(),
            sections: Vec::default(),
            config,
        })
    }

    fn get_remote_details(repository: &Repository) -> Result<(String, String), ChangeLogError> {
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

    // /// set header
    // pub fn set_header(&mut self, value: &str) -> &mut Self {
    //     self.header = value.to_string();
    //     self
    // }

    /// Build the sections of the change log
    pub fn build(&mut self) -> Result<&mut Self, ChangeLogError> {
        let mut tags = Vec::new();

        self.repository.tag_foreach(|id, name| {
            let name = String::from_utf8(name.to_vec()).unwrap_or("invalid utf8".to_string());
            log::debug!("processing {name} as a tag");
            let mut tag_builder = Tag::builder(id, name, self.repository);
            let tag = tag_builder
                .get_semver(self.config.release_pattern())
                .get_date()
                .build();
            log::debug!(
                "Identified `{}` as version `{:?}`",
                tag.name(),
                if tag.is_version_tag() {
                    tag.version().unwrap().to_string()
                } else {
                    "NOT A VERSION".to_string()
                }
            );
            tags.push(tag);
            true
        })?;

        log::debug!(
            "Tags:`{}`",
            tags.iter()
                .map(|t| t.name().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut revwalk = self.repository.revwalk()?;

        revwalk.set_sorting(git2::Sort::NONE)?;
        revwalk.push_head()?;
        log::debug!("starting the walk from the HEAD");
        // log::debug!("the reference to walk back to is: `{reference}`");
        // revwalk.hide_ref(reference)?;

        let mut current_section = Section::new(None);

        for oid in revwalk.flatten() {
            if let Some(tag) = tags.iter().find(|t| t.id() == &oid) {
                log::debug!("found the tag: `{tag}`");
                log::debug!("{}", current_section.report_status());
                self.sections.push(current_section);
                current_section = Section::new(Some(tag.clone()));
            };

            let Ok(commit) = self.repository.find_commit(oid) else {
                continue;
            };

            let Some(summary) = commit.summary() else {
                continue;
            };
            let body = commit.body();
            log::trace!("Found commit with Summary:\t`{summary}.");
            current_section.add_commit(Some(summary), body);
        }
        log::debug!("{}", current_section.report_status());
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
            log::debug!("Processing `{}` sections", self.sections.len());
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

        let footer = footer_vec.join("\n");
        println!("{}{}{}", self.header, report, footer)
    }
}
