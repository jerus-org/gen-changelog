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

/// Regular expression pattern for matching GitHub repository URLs.
///
/// Supports both HTTPS and SSH formats:
/// - HTTPS: `https://github.com/owner/repo.git`
/// - SSH: `git@github.com:owner/repo.git`
///
/// Captures named groups:
/// - `owner`: The repository owner/organization name must be valid GitHub owner
/// - `repo`: The repository name
static REMOTE: Lazy<Regex> = lazy_regex!(
    r"^((https://github\.com/)|(git@github.com:))(?P<owner>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,37}[a-zA-Z0-9])?)/(?P<repo>[a-zA-Z0-9_][a-zA-Z0-9_-]+[a-zA-Z0-9_])\.git$"
);

/// The main ChangeLog structure that represents a complete changelog document.
///
/// A changelog consists of:
/// - A header with title and description
/// - Multiple sections representing different versions/releases
/// - Links for navigation and references
///
/// # Example
///
/// ```rust
/// use change_log::ChangeLog;
///
/// let changelog = ChangeLog::builder()
///     .with_header("My Project", &["A description of the project"])
///     .build();
///
/// // Save to CHANGELOG.md
/// changelog.save().expect("Failed to save changelog");
/// ```
#[derive(Debug, Clone)]
pub struct ChangeLog {
    /// The changelog header containing title and description
    header: Header,
    /// Collection of version sections in chronological order
    sections: Vec<Section>,
    /// Reference links used throughout the changelog
    links: Vec<Link>,
}

impl ChangeLog {
    /// Creates a new ChangeLogBuilder for constructing a ChangeLog.
    ///
    /// This is the preferred way to create a new ChangeLog instance,
    /// as it provides a fluent interface for configuration.
    ///
    /// # Returns
    ///
    /// A new `ChangeLogBuilder` instance with default values.
    ///
    /// # Example
    ///
    /// ```rust
    /// let changelog = ChangeLog::builder()
    ///     .with_header("My Project", &["Project description"])
    ///     .build();
    /// ```
    pub fn builder() -> ChangeLogBuilder {
        ChangeLogBuilder::new()
    }

    /// Writes the changelog to a file named "CHANGELOG.md" in the current directory.
    ///
    /// This method serializes the entire changelog structure to markdown format
    /// and saves it to the filesystem.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the file was written successfully
    /// - `Err(Error)` if there was an I/O error during writing
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file cannot be created or written to
    /// - There are insufficient permissions to write to the directory
    /// - The disk is full
    ///
    /// # Example
    ///
    /// ```rust
    /// let changelog = ChangeLog::builder().build();
    /// changelog.save().expect("Failed to save changelog");
    /// ```
    pub fn save(&self) -> Result<(), Error> {
        std::fs::write("CHANGELOG.md", self.to_string().as_str())?;
        Ok(())
    }
}

impl Display for ChangeLog {
    /// Formats the ChangeLog as a markdown string.
    ///
    /// The output format follows the Keep a Changelog specification:
    /// - Header section with title and description
    /// - Version sections in reverse chronological order
    /// - Reference links at the bottom
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

/// Builder pattern implementation for constructing ChangeLog instances.
///
/// The ChangeLogBuilder provides a fluent interface for configuring and
/// building changelog documents. It handles Git repository analysis,
/// version tag processing, and link generation.
///
/// # Example
///
/// ```rust
/// use git2::Repository;
/// use change_log::{ChangeLog, ChangeLogConfig};
///
/// let repo = Repository::open(".")?;
/// let config = ChangeLogConfig::default();
///
/// let changelog = ChangeLog::builder()
///     .with_config(config)
///     .with_header("My Project", &["A great project"])
///     .with_repository(&repo)?
///     .build();
/// ```
pub struct ChangeLogBuilder {
    /// Repository owner (GitHub username or organization)
    owner: String,
    /// Repository name
    repo: String,
    /// Changelog header
    header: Header,
    /// Version sections
    sections: Vec<Section>,
    /// Whether to include summary information
    summary_flag: bool,
    /// Reference links
    links: Vec<Link>,
    /// Configuration for changelog generation
    config: ChangeLogConfig,
}

impl Debug for ChangeLogBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChangeLogBuilder")
            .field("owner", &self.owner)
            .field("repo", &self.repo)
            .field("header", &self.header)
            .field("sections", &self.sections)
            .field("links", &self.links)
            .finish()
    }
}

impl ChangeLogBuilder {
    /// Creates a new ChangeLogBuilder with default values.
    ///
    /// All fields are initialized to their default states:
    /// - Empty owner and repo strings
    /// - Default header
    /// - Empty sections and links vectors
    /// - Summary flag set to false
    /// - Default configuration
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

    /// Constructs the final ChangeLog from the builder configuration.
    ///
    /// This method consumes the builder's current state and creates
    /// an immutable ChangeLog instance.
    ///
    /// # Returns
    ///
    /// A new `ChangeLog` instance with the current builder configuration.
    pub fn build(&self) -> ChangeLog {
        ChangeLog {
            header: self.header.clone(),
            sections: self.sections.clone(),
            links: self.links.clone(),
        }
    }

    /// Sets a custom configuration for changelog generation.
    ///
    /// The configuration controls various aspects of changelog generation
    /// including section display limits, grouping rules, and formatting options.
    ///
    /// # Arguments
    ///
    /// * `config` - The custom ChangeLogConfig to use
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use change_log::{ChangeLog, ChangeLogConfig};
    ///
    /// let config = ChangeLogConfig::default();
    /// let builder = ChangeLog::builder()
    ///     .with_config(config);
    /// ```
    pub fn with_config(&mut self, config: ChangeLogConfig) -> &mut Self {
        self.config = config;
        log::trace!("current config: {:?}", self.config);
        self
    }

    /// Sets the changelog header with a title and description paragraphs.
    ///
    /// The header appears at the top of the changelog and typically contains
    /// the project name and a brief description.
    ///
    /// # Arguments
    ///
    /// * `title` - The main title for the changelog
    /// * `paragraphs` - Array of description paragraphs
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = ChangeLog::builder()
    ///     .with_header(
    ///         "My Awesome Project",
    ///         &["This project does amazing things", "Version history below"]
    ///     );
    /// ```
    pub fn with_header(&mut self, title: &str, paragraphs: &[&str]) -> &mut Self {
        self.header = Header::new(title, paragraphs);
        self
    }

    /// Enables or disables summary information in changelog sections.
    ///
    /// When enabled, sections may include additional summary statistics
    /// or metadata about the changes in that version.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to include summary information
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn with_summary_flag(&mut self, value: bool) -> &mut Self {
        self.summary_flag = value;
        self
    }

    /// Analyses a Git repository to populate changelog sections and links.
    ///
    /// This method performs the core changelog generation logic:
    /// 1. Extracts remote repository details (owner/repo)
    /// 2. Identifies and sorts version tags
    /// 3. Creates sections for each version
    /// 4. Generates comparison links between versions
    ///
    /// # Arguments
    ///
    /// * `repository` - A reference to the Git repository to analyze
    ///
    /// # Returns
    ///
    /// - `Ok(&mut Self)` for method chaining if successful
    /// - `Err(Error)` if repository analysis fails
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The repository has no remote origin configured
    /// - The remote URL is not a recognized GitHub format
    /// - Git operations fail (e.g., walking commits, reading tags)
    /// - Repository access permissions are insufficient
    ///
    /// # Example
    ///
    /// ```rust
    /// use git2::Repository;
    ///
    /// let repo = Repository::open(".")?;
    /// let changelog = ChangeLog::builder()
    ///     .with_repository(&repo)?
    ///     .build();
    /// ```
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

    /// Updates the first (unreleased) section to represent the next version.
    ///
    /// This is typically used when preparing a release to convert the
    /// "Unreleased" section into a specific version section.
    ///
    /// # Arguments
    ///
    /// * `next_version` - Optional version string to set for the unreleased section
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// let version = String::from("1.2.0");
    /// let builder = ChangeLog::builder()
    ///     .update_unreleased_to_next_version(Some(&version));
    /// ```
    pub fn update_unreleased_to_next_version(
        &mut self,
        next_version: Option<&String>,
    ) -> &mut Self {
        if !self.sections.is_empty() {
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
        }
        self
    }
}

impl ChangeLogBuilder {
    /// Extracts GitHub repository owner and name from the remote origin URL.
    ///
    /// This method reads the Git configuration to find the remote origin URL
    /// and parses it to extract the repository owner and name using the
    /// `REMOTE` regex pattern.
    ///
    /// # Arguments
    ///
    /// * `repository` - The Git repository to analyze
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the remote details were successfully extracted
    /// - `Err(Error)` if extraction fails
    ///
    /// # Errors
    ///
    /// - `Error::UrlNotFound` - No remote origin URL configured
    /// - `Error::CapturesNotFound` - URL doesn't match GitHub format
    /// - `Error::OwnerNotFound` - Owner not captured from URL
    /// - `Error::RepoNotFound` - Repository name not captured from URL
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

    /// Creates and stores appropriate links based on the version walk setup.
    ///
    /// Different link types are generated depending on the version range:
    /// - Unreleased: Links to commits on main branch
    /// - Version comparisons: Links to GitHub compare view
    /// - Initial version: Links to release tag
    ///
    /// # Arguments
    ///
    /// * `setup` - The walk setup configuration determining link type
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

    /// Retrieves and processes version tags from the Git repository.
    ///
    /// This method:
    /// 1. Iterates through all repository tags
    /// 2. Identifies which tags represent versions using semantic versioning
    /// 3. Sorts version tags in reverse chronological order (newest first)
    ///
    /// # Arguments
    ///
    /// * `repository` - The Git repository to analyze
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Tag>)` - Vector of version tags sorted newest to oldest
    /// - `Err(Error)` - If tag processing fails
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
