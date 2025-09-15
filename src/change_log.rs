mod header;
mod link;
mod section;
mod tag;

use std::{
    cmp::min,
    fmt::{Debug, Display},
};

use git2::Repository;
use header::Header;
use lazy_regex::{Lazy, Regex, lazy_regex};
use link::Link;
use section::{Section, WalkSetup};
use tag::Tag;

use crate::{ChangeLogConfig, Error, change_log_config::DisplaySections};

pub static REMOTE: Lazy<Regex> = lazy_regex!(
    r"^((https://github\.com/)|(git@github.com:))(?P<owner>[a-z\-|A-Z]+)/(?P<repo>[a-z\-_A-Z]+)\.git$$"
);

/// ChangeLog main struct
#[derive(Debug, Clone)]
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

    /// Write the changelog to the root directory
    pub fn save(&self) -> Result<(), Error> {
        std::fs::write("CHANGELOG.md", self.to_string().as_str())?;
        Ok(())
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
        writeln!(f, "{}\n{}{}", self.header, sections, links)
    }
}

/// ChangeLogBuilder struct
pub struct ChangeLogBuilder {
    owner: String,
    repo: String,
    header: Header,
    sections: Vec<Section>,
    summary_flag: bool,
    links: Vec<Link>,
    config: ChangeLogConfig,
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
        ChangeLogBuilder {
            owner: String::default(),
            repo: String::default(),
            header: Header::default(),
            links: Vec::new(),
            sections: Vec::default(),
            summary_flag: bool::default(),
            config: ChangeLogConfig::default(),
        }
    }

    /// build a ChangeLog struct from the ChangeLogBuilder
    pub fn build(&self) -> ChangeLog {
        ChangeLog {
            header: self.header.clone(),
            sections: self.sections.clone(),
            links: self.links.clone(),
        }
    }

    /// Replace default config with custom config
    pub fn with_config(&mut self, config: ChangeLogConfig) -> &mut Self {
        self.config = config;
        log::trace!("current config: {:?}", self.config);
        self
    }

    /// set header
    pub fn with_header(&mut self, title: &str, paragraphs: &[&str]) -> &mut Self {
        self.header = Header::new(title, paragraphs);
        self
    }

    /// set header
    pub fn with_summary_flag(&mut self, value: bool) -> &mut Self {
        self.summary_flag = value;
        self
    }

    /// Add sections  and links to the change log
    pub fn with_repository(&mut self, repository: &Repository) -> Result<&mut Self, Error> {
        self.get_remote_details(repository)?;

        let version_tags = self.get_version_tags(repository)?;

        let section_limit = match self.config.display_sections() {
            DisplaySections::All => min((version_tags.len() + 1) as u8, u8::MAX),
            DisplaySections::One => 1,
            DisplaySections::Custom(n) => min((version_tags.len() + 1) as u8, *n),
        };

        let mut revwalk = repository.revwalk()?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        let groups_mapping = self.config.groups_mapping();

        let mut current_section = Section::new(
            None,
            self.config.headings(),
            self.summary_flag,
            &groups_mapping,
        );

        // Case where no release has been made - no version tags
        if version_tags.is_empty() {
            let setup = WalkSetup::NoReleases;
            current_section.walk_repository(&setup, repository, &mut revwalk)?;
            self.sections.push(current_section);
            self.set_link(&setup);
        } else {
            // get the unreleased
            let setup = WalkSetup::HeadToRelease(version_tags.first().unwrap());
            current_section.walk_repository(&setup, repository, &mut revwalk)?;
            self.sections.push(current_section);
            self.set_link(&setup);

            // get the releases
            let mut section_count = 1;
            let mut peekable_tags = version_tags.iter().peekable();
            loop {
                if section_count >= section_limit {
                    break;
                }
                let Some(tag) = peekable_tags.next() else {
                    break;
                };

                let mut section = Section::new(
                    Some(tag.clone()),
                    self.config.headings(),
                    self.summary_flag,
                    &groups_mapping,
                );

                let next_tag = peekable_tags.peek();

                if let Some(next_tag) = next_tag {
                    let setup = WalkSetup::FromReleaseToRelease(tag, next_tag);
                    section.walk_repository(&setup, repository, &mut revwalk)?;
                    self.set_link(&setup);
                } else {
                    let setup = WalkSetup::ReleaseToStart(tag);
                    section.walk_repository(&setup, repository, &mut revwalk)?;
                    self.set_link(&setup);
                }
                self.sections.push(section);
                section_count += 1;
            }
        }

        Ok(self)
    }

    /// Update unreleased to the next version
    pub fn update_unreleased_to_next_version(
        &mut self,
        next_version: Option<&String>,
    ) -> &mut Self {
        if let Some(nv) = next_version {
            log::debug!(
                "Setting unreleased section `{}` to `{nv}`",
                self.sections[0].header()
            );

            self.sections[0].set_version(nv);

            log::debug!(
                "Updated unreleased section tag is `{:?}`.",
                self.sections[0].tag()
            );
        }
        self
    }
}

impl ChangeLogBuilder {
    fn get_remote_details(&mut self, repository: &Repository) -> Result<(), Error> {
        let config = repository.config()?;
        let url = config.get_entry("remote.origin.url")?;
        let Some(haystack) = url.value() else {
            return Err(Error::UrlNotFound);
        };

        let captures = REMOTE.captures(haystack);

        let Some(caps) = captures else {
            return Err(Error::CapturesNotFound);
        };

        let Some(owner) = caps.name("owner") else {
            return Err(Error::OwnerNotFound);
        };
        let Some(repo) = caps.name("repo") else {
            return Err(Error::RepoNotFound);
        };

        self.owner = owner.as_str().to_string();
        self.repo = repo.as_str().to_string();

        Ok(())
    }

    fn set_link(&mut self, setup: &WalkSetup) {
        match setup {
            WalkSetup::NoReleases => {
                let url = format!(
                    "https://github.com/{}/{}/commits/main/",
                    self.owner, self.repo
                );

                let link = Link::new("Unreleased", &url).unwrap();
                self.links.push(link)
            }

            WalkSetup::HeadToRelease(tag) => {
                let tag_version = tag.version().unwrap().to_string();
                let url = format!(
                    "https://github.com/{}/{}/compare/v{}...HEAD",
                    self.owner, self.repo, tag_version
                );
                let link = Link::new("Unreleased", &url).unwrap();
                log::debug!("Head to release link: {link}");
                self.links.push(link)
            }

            WalkSetup::FromReleaseToRelease(tag, next_tag) => {
                let tag_version = tag.version().unwrap().to_string();
                let next_tag_version = next_tag.version().unwrap().to_string();
                let url = format!(
                    "https://github.com/{}/{}/compare/v{}...v{}",
                    self.owner, self.repo, next_tag_version, tag_version
                );

                let link = Link::new(&tag_version, &url).unwrap();
                self.links.push(link)
            }
            WalkSetup::ReleaseToStart(tag) => {
                let tag_version = tag.version().unwrap().to_string();
                let url = format!(
                    "https://github.com/{}/{}/releases/tag/v{}",
                    self.owner, self.repo, tag_version
                );

                let link = Link::new(&tag_version, &url).unwrap();
                self.links.push(link)
            }
        }
    }

    fn get_version_tags(&self, repository: &Repository) -> Result<Vec<Tag>, Error> {
        let mut tags = Vec::new();

        repository.tag_foreach(|id, name| {
            let name = String::from_utf8(name.to_vec()).unwrap_or("invalid utf8".to_string());
            log::trace!("processing {name} as a tag");
            let mut tag_builder = Tag::builder(Some(id), name, repository);
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
        log::trace!("Identified {} version tags.", version_tags.len());
        log::trace!(
            "Tags:`{}`",
            tags.iter()
                .map(|t| t.name().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        Ok(version_tags)
    }
}
