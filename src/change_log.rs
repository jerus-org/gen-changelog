mod header;
mod link;
mod section;
mod tag;

use std::fmt::{Debug, Display};

use git2::Repository;
use header::Header;
use lazy_regex::{Lazy, Regex, lazy_regex};
use link::Link;
use section::{Section, WalkSetup};
use tag::Tag;
use thiserror::Error;

use crate::Config;

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
pub struct ChangeLog {
    header: Header,
    sections: Vec<Section>,
    links: Vec<Link>,
}

impl ChangeLog {
    /// create new ChangeLog struct
    pub fn builder() -> ChangeLogBuilder {
        ChangeLogBuilder::new()
    }
}

impl Display for ChangeLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sections = self
            .sections
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        let links = self.links.iter().map(|s| s.to_string()).collect::<String>();
        writeln!(f, "{}{}{}", self.header, sections, links)
    }
}

/// ChangeLogBuilder struct
pub struct ChangeLogBuilder {
    owner: String,
    repo: String,
    header: Header,
    sections: Vec<Section>,
    links: Vec<Link>,
    config: Config,
}

impl Debug for ChangeLogBuilder {
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

impl ChangeLogBuilder {
    /// create new ChangeLogBuilder struct
    pub(crate) fn new() -> ChangeLogBuilder {
        // let (owner, repo) = ChangeLogBuilder::get_remote_details(repository)?;

        ChangeLogBuilder {
            owner: String::default(),
            repo: String::default(),
            header: Header::default(),
            links: Vec::new(),
            sections: Vec::default(),
            config: Config::default(),
        }
    }

    pub fn build(&self) -> ChangeLog {
        ChangeLog {
            header: self.header.clone(),
            sections: self.sections.clone(),
            links: self.links.clone(),
        }
    }

    /// Replace default config with custom config
    pub fn with_config(&mut self, config: Config) -> &mut Self {
        self.config = config;
        log::debug!("current config: {:?}", self.config);
        self
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

    /// set header
    pub fn with_header(&mut self, title: &str, paragraphs: &[&str]) -> &mut Self {
        self.header = Header::new(title, paragraphs);
        self
    }

    fn get_version_tags(&self, repository: &Repository) -> Result<Vec<Tag>, ChangeLogError> {
        let mut tags = Vec::new();

        repository.tag_foreach(|id, name| {
            let name = String::from_utf8(name.to_vec()).unwrap_or("invalid utf8".to_string());
            log::trace!("processing {name} as a tag");
            let mut tag_builder = Tag::builder(id, name, repository);
            let tag = tag_builder
                .get_semver(self.config.release_pattern())
                .get_date()
                .build();
            log::trace!(
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

        let mut version_tags = tags.clone();
        version_tags.retain(|t| t.is_version_tag());
        version_tags.sort_by_key(|k| k.version().unwrap().clone());
        version_tags.reverse();
        log::debug!("Identified {} version tags.", version_tags.len());
        log::debug!(
            "Tags:`{}`",
            tags.iter()
                .map(|t| t.name().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        Ok(version_tags)
    }

    /// Add sections  and links to the change log
    pub fn with_repository(
        &mut self,
        repository: &Repository,
    ) -> Result<&mut Self, ChangeLogError> {
        let version_tags = self.get_version_tags(repository)?;

        let mut revwalk = repository.revwalk()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        let mut current_section = Section::new(None, self.config.headings());

        // Case where no release has been made - no version tags
        if version_tags.is_empty() {
            current_section.walk_repository(WalkSetup::NoReleases, repository, &mut revwalk)?;
            self.sections.push(current_section);
        } else {
            // get the unreleased
            let setup = WalkSetup::HeadToRelease(version_tags.first().unwrap());
            current_section.walk_repository(setup, repository, &mut revwalk)?;
            self.sections.push(current_section);

            // get the releases
            let mut peekable_tags = version_tags.iter().peekable();
            loop {
                let Some(tag) = peekable_tags.next() else {
                    break;
                };

                let mut section = Section::new(Some(tag.clone()), self.config.headings());

                let next_tag = peekable_tags.peek();

                if let Some(next_tag) = next_tag {
                    let setup = WalkSetup::FromReleaseToRelease(tag, next_tag);
                    section.walk_repository(setup, repository, &mut revwalk)?;
                } else {
                    let setup = WalkSetup::ReleaseToStart(tag);
                    section.walk_repository(setup, repository, &mut revwalk)?;
                }
                self.sections.push(section);
            }
        };

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
