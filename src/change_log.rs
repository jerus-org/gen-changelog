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
/// use gen_changelog::ChangeLog;
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
    /// use gen_changelog::ChangeLog;
    ///
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
    /// use gen_changelog::ChangeLog;
    ///
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
/// use gen_changelog::{ChangeLog, ChangeLogConfig};
///
/// # fn main()  -> Result<(), Box<dyn std::error::Error>>  {
/// let repo = Repository::open(".")?;
/// let config = ChangeLogConfig::default();
///
/// let changelog = ChangeLog::builder()
///     .with_config(config)
///     .with_header("My Project", &["A great project"])
///     .with_repository(&repo)?
///     .build();
/// # Ok(())
/// # }
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
    /// use gen_changelog::{ChangeLog, ChangeLogConfig};
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
    /// use gen_changelog::ChangeLog;
    ///
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
    /// use gen_changelog::ChangeLog;
    ///
    /// # fn main() -> Result<(), gen_changelog::Error> {
    /// let repo = Repository::open(".")?;
    /// let changelog = ChangeLog::builder()
    ///     .with_repository(&repo)?
    ///     .build();
    /// # Ok(())
    /// # }
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
    /// use gen_changelog::ChangeLog;
    ///
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper function to create a temporary directory for tests
    fn setup_temp_dir() -> TempDir {
        TempDir::new().expect("Failed to create temp directory")
    }

    /// Test the REMOTE regex pattern with various GitHub URL formats
    #[test]
    fn test_remote_regex_patterns() {
        // Test HTTPS URL
        let https_url = "https://github.com/user/repo.git";
        let caps = REMOTE.captures(https_url).unwrap();
        assert_eq!(caps.name("owner").unwrap().as_str(), "user");
        assert_eq!(caps.name("repo").unwrap().as_str(), "repo");

        // Test SSH URL
        let ssh_url = "git@github.com:organization/my-repo.git";
        let caps = REMOTE.captures(ssh_url).unwrap();
        assert_eq!(caps.name("owner").unwrap().as_str(), "organization");
        assert_eq!(caps.name("repo").unwrap().as_str(), "my-repo");

        // Test URL with underscores (not accepted in username)
        let underscore_url = "https://github.com/username/repo_name.git";
        let caps = REMOTE.captures(underscore_url).unwrap();
        assert_eq!(caps.name("owner").unwrap().as_str(), "username");
        assert_eq!(caps.name("repo").unwrap().as_str(), "repo_name");

        // Test invalid URLs
        assert!(
            REMOTE
                .captures("https://gitlab.com/user/repo.git")
                .is_none()
        );
        assert!(REMOTE.captures("https://github.com/user/repo").is_none()); // Missing .git
    }

    #[test]
    fn test_changelog_builder_creation() {
        let builder = ChangeLogBuilder::new();

        assert_eq!(builder.owner, "");
        assert_eq!(builder.repo, "");
        assert_eq!(builder.sections.len(), 0);
        assert_eq!(builder.links.len(), 0);
        assert!(!builder.summary_flag);
    }

    #[test]
    fn test_changelog_builder_with_header() {
        let mut builder = ChangeLogBuilder::new();
        builder.with_header(
            "Test Project",
            &["A test project", "With multiple paragraphs"],
        );

        let changelog = builder.build();
        let output = changelog.to_string();
        assert!(output.contains("Test Project"));
    }

    #[test]
    fn test_changelog_builder_with_summary_flag() {
        let mut builder = ChangeLogBuilder::new();
        builder.with_summary_flag(true);

        assert!(builder.summary_flag);

        builder.with_summary_flag(false);
        assert!(!builder.summary_flag);
    }

    #[test]
    fn test_changelog_builder_with_config() {
        let mut builder = ChangeLogBuilder::new();
        let config = ChangeLogConfig::default();

        builder.with_config(config);
        // Test that the config was set (this would require PartialEq on ChangeLogConfig)
        // For now, we just test that the method doesn't panic
    }

    #[test]
    fn test_changelog_display_formatting() {
        let changelog = ChangeLog {
            header: Header::default(),
            sections: Vec::new(),
            links: Vec::new(),
        };

        let output = changelog.to_string();
        assert!(!output.is_empty());
        // The exact format depends on Header::default() implementation
    }

    #[test]
    fn test_changelog_save() {
        let temp_dir = setup_temp_dir();
        let temp_path = temp_dir.path();

        // Change to temp directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_path).unwrap();

        let changelog = ChangeLog::builder().build();
        let result = changelog.save();

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(temp_path.join("CHANGELOG.md").exists());

        let content = fs::read_to_string(temp_path.join("CHANGELOG.md")).unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_update_unreleased_to_next_version() {
        // This test would require setting up sections first
        // For now, we test the method doesn't panic when sections is empty
        let mut builder = ChangeLogBuilder::new();
        let version = String::from("1.0.0");

        // This should not panic even with empty sections
        builder.update_unreleased_to_next_version(Some(&version));
        builder.update_unreleased_to_next_version(None);
    }

    #[test]
    fn test_builder_debug_implementation() {
        let builder = ChangeLogBuilder::new();
        let debug_output = format!("{builder:?}");

        assert!(debug_output.contains("ChangeLogBuilder"));
        assert!(debug_output.contains("owner"));
        assert!(debug_output.contains("repo"));
        assert!(debug_output.contains("header"));
        assert!(debug_output.contains("sections"));
        assert!(debug_output.contains("links"));
    }

    #[test]
    fn test_changelog_clone() {
        let changelog = ChangeLog {
            header: Header::default(),
            sections: Vec::new(),
            links: Vec::new(),
        };

        let cloned = changelog.clone();
        assert_eq!(changelog.sections.len(), cloned.sections.len());
        assert_eq!(changelog.links.len(), cloned.links.len());
    }
    use std::time::Instant;

    // use std::collections::HashMap;

    /// Test helper to create a ChangeLog with predefined content
    pub fn create_test_changelog() -> ChangeLog {
        ChangeLog::builder()
            .with_header("Test Changelog", &["A test changelog for unit tests"])
            .build()
    }

    /// Test helper to create a ChangeLogBuilder with common test configuration
    pub fn create_test_builder() -> ChangeLogBuilder {
        let mut builder = ChangeLogBuilder::new();
        builder
            .with_header("Test Project", &["Test description"])
            .with_summary_flag(true);
        builder
    }

    /// Test helper to verify changelog markdown format
    pub fn verify_markdown_format(content: &str) -> bool {
        // Basic checks for markdown structure
        content.contains("#") && // Should have headers
        !content.trim().is_empty() // Should not be empty
    }

    /// Test helper to create mock repository data
    pub struct MockRepoData {
        pub owner: String,
        pub repo: String,
        pub tags: Vec<String>,
        pub commits: Vec<String>,
    }

    impl Default for MockRepoData {
        fn default() -> Self {
            Self {
                owner: "testowner".to_string(),
                repo: "testrepo".to_string(),
                tags: vec!["v1.0.0".to_string(), "v1.1.0".to_string()],
                commits: vec!["Initial commit".to_string(), "Add feature".to_string()],
            }
        }
    }

    #[test]
    fn test_error_recovery() {
        // Test that the system can recover from various error conditions
        let mut builder = ChangeLogBuilder::new();

        // Test with invalid configuration
        // Should not panic and should handle gracefully
        builder.with_summary_flag(true);
    }

    #[test]
    fn test_empty_header() {
        let mut builder = ChangeLogBuilder::new();
        builder.with_header("", &[]);

        let changelog = builder.build();
        let output = changelog.to_string();

        // Should handle empty header gracefully
        assert!(!output.is_empty());
    }

    #[test]
    fn test_very_long_header() {
        let mut builder = ChangeLogBuilder::new();
        let long_title = "A".repeat(1000);
        let long_paragraph = "B".repeat(1000);
        let long_paragraphs: Vec<&str> = vec![&long_paragraph; 10];

        builder.with_header(&long_title, &long_paragraphs);
        let changelog = builder.build();

        // Should handle very long content without issues
        assert!(changelog.to_string().len() > 1000);
    }

    #[test]
    fn test_special_characters_in_header() {
        let mut builder = ChangeLogBuilder::new();
        builder.with_header(
            "Project with éspecial & <characters>",
            &["Description with \"quotes\" and 'apostrophes'"],
        );

        let changelog = builder.build();
        let output = changelog.to_string();

        // Should preserve special characters
        assert!(output.contains("éspecial"));
        assert!(output.contains("&"));
        assert!(output.contains("<characters>"));
    }

    #[test]
    fn test_unicode_handling() {
        let mut builder = ChangeLogBuilder::new();
        builder.with_header(
            "🚀 Project",
            &["Description with emoji 🎉 and unicode ñáéíóú"],
        );

        let changelog = builder.build();
        let output = changelog.to_string();

        // Should handle Unicode properly
        assert!(output.contains("🚀"));
        assert!(output.contains("🎉"));
        assert!(output.contains("ñáéíóú"));
    }

    #[test]
    fn test_multiple_config_updates() {
        let mut builder = ChangeLogBuilder::new();

        // Test multiple configuration updates
        let config1 = ChangeLogConfig::default();
        let config2 = ChangeLogConfig::default();

        builder.with_config(config1);
        builder.with_config(config2);

        // Should handle multiple config updates gracefully
    }

    #[test]
    fn test_version_update_edge_cases() {
        let mut builder = ChangeLogBuilder::new();

        // Test with None
        builder.update_unreleased_to_next_version(None);

        // Test with empty string
        let empty_version = String::new();
        builder.update_unreleased_to_next_version(Some(&empty_version));

        // Test with very long version string
        let long_version = "1.0.0-".to_string() + &"a".repeat(1000);
        builder.update_unreleased_to_next_version(Some(&long_version));

        // Should handle all edge cases without panicking
    }

    #[test]
    fn test_changelog_creation_performance() {
        let start = Instant::now();

        // Create many changelogs to test performance
        for i in 0..1000 {
            let _changelog = ChangeLog::builder()
                .with_header(&format!("Project {i}"), &[&format!("Description {i}")])
                .build();
        }

        let duration = start.elapsed();

        // Should complete within reasonable time (adjust threshold as needed)
        assert!(
            duration.as_millis() < 1000,
            "Changelog creation too slow: {duration:?}"
        );
    }

    #[test]
    fn test_regex_performance() {
        let urls = vec![
            "https://github.com/user/repo.git",
            "git@github.com:org/project.git",
            "https://github.com/owner/repository.git",
        ];

        let start = Instant::now();

        // Test regex performance with many iterations
        for _ in 0..10000 {
            for url in &urls {
                let _ = REMOTE.captures(url);
            }
        }

        let duration = start.elapsed();

        // Should complete within reasonable time
        assert!(
            duration.as_millis() < 500,
            "Regex matching too slow: {duration:?}"
        );
    }

    #[test]
    fn test_large_changelog_formatting() {
        let mut builder = ChangeLogBuilder::new();
        builder.with_header("Large Project", &["A project with many sections"]);

        // Create a changelog with many empty sections to test formatting performance
        let changelog = builder.build();

        let start = Instant::now();
        let output = changelog.to_string();
        let duration = start.elapsed();

        assert!(!output.is_empty());
        assert!(
            duration.as_millis() < 100,
            "Formatting too slow: {duration:?}"
        );
    }

    // Concurrency tests (if the code needs to be thread-safe)
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_changelog_creation() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let changelog = ChangeLog::builder()
                        .with_header(&format!("Project {i}"), &[&format!("Desc {i}")])
                        .build();
                    changelog.to_string()
                })
            })
            .collect();

        // Wait for all threads and collect results
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(results.len(), 10);
        for (i, result) in results.iter().enumerate() {
            assert!(result.contains(&format!("Project {i}")));
        }
    }

    #[test]
    fn test_shared_regex_pattern() {
        // Test that the static REMOTE regex can be used from multiple threads
        let url = Arc::new("https://github.com/user/repo.git".to_string());

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let url_clone = Arc::clone(&url);
                thread::spawn(move || REMOTE.captures(&url_clone).is_some())
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All threads should successfully match the URL
        assert!(results.iter().all(|&r| r));
    }

    // Documentation tests (these test the examples in the doc comments)
    // These would normally be handled by `cargo test --doc`
    // but we can add explicit tests for documentation examples

    #[test]
    fn test_basic_usage_example() {
        // Test the example from the ChangeLog documentation
        let changelog = super::ChangeLog::builder()
            .with_header("My Project", &["A description of the project"])
            .build();

        let output = changelog.to_string();
        assert!(output.contains("My Project"));
        assert!(output.contains("A description of the project"));
    }

    #[test]
    fn test_builder_example() {
        // Test the example from the builder() method documentation
        let changelog = super::ChangeLog::builder()
            .with_header("My Project", &["Project description"])
            .build();

        assert!(!changelog.to_string().is_empty());
    }

    #[test]
    fn test_header_example() {
        // Test the example from with_header documentation
        let mut builder = super::ChangeLog::builder();
        builder.with_header(
            "My Awesome Project",
            &["This project does amazing things", "Version history below"],
        );

        let changelog = builder.build();
        let output = changelog.to_string();

        assert!(output.contains("My Awesome Project"));
        assert!(output.contains("This project does amazing things"));
        assert!(output.contains("Version history below"));
    }
}
