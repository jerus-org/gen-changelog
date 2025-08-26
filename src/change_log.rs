mod section;
mod tag;

use git2::Repository;
use section::Section;
use std::path::{Path, PathBuf};
use thiserror::Error;

use tag::Tag;

const DEFAULT_HEADER: &str = r##"
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

"##;

const DEFAULT_FOOTER: &str = r##""##;

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
    header: String,
    sections: Vec<Section>,
    footer: String,
}

impl Default for ChangeLog {
    fn default() -> Self {
        ChangeLog {
            repo_dir: PathBuf::new().join("."),
            header: DEFAULT_HEADER.to_string(),
            footer: DEFAULT_FOOTER.to_string(),
            sections: Vec::default(),
        }
    }
}

impl ChangeLog {
    /// create new ChangeLog struct
    pub fn new(repo_dir: &Path) -> ChangeLog {
        ChangeLog {
            repo_dir: repo_dir.to_path_buf(),
            ..Default::default()
        }
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

        println!("Tags:\n{tags:#?}");

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
            println!("Found commit with Summary:\t`{summary}.");
            current_section.add_commit(Some(summary), body);
        }
        println!("{}", current_section.report_status());
        self.sections.push(current_section);

        Ok(self)
    }

    /// Print the change log to standard out
    pub fn print(&self) {
        let mut report = String::new();

        for section in &self.sections {
            let rep = section.section_markdown();
            report.push_str(&rep);
        }

        println!("{}{}{}", self.header, report, self.footer)
    }
}
