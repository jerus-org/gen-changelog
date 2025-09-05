use std::fmt::Display;

use chrono::{DateTime, Utc};
use git2::{ObjectType, Oid, Repository};
use lazy_regex::{Lazy, Regex, lazy_regex};
use semver::Version;
use thiserror::Error;

use crate::change_log_config::ReleasePattern;

pub static PREFIX: Lazy<Regex> = lazy_regex!(
    r#"(?P<prefix>\w+)(?P<semver>(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(?P<pre>-[a-z\.A-Z0-9]+)?(?P<build>\+[0-9A-Za-z-\.]+)?)"#
);
pub static PACKAGE_PREFIX: Lazy<Regex> = lazy_regex!(
    r#"(?P<package>(([-_]?\w+)+))-(?P<prefix>\w+)(?P<semver>(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(?P<pre>-[a-z\.A-Z0-9]+)?(?P<build>\+[0-9A-Za-z-\.]+)?)"#
);

#[derive(Debug, Clone)]
pub(crate) struct Tag {
    id: Oid,
    name: String,
    package: Option<String>,
    semver: Option<Version>,
    date: Option<DateTime<Utc>>,
}

impl Tag {
    pub(crate) fn builder<S: Display>(id: Oid, name: S, repo: &Repository) -> TagBuilder<'_> {
        let name = name.to_string();

        TagBuilder {
            id,
            repo,
            name,
            package: None,
            semver: None,
            date: None,
        }
    }

    pub(crate) fn id(&self) -> &Oid {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn version(&self) -> Option<&Version> {
        self.semver.as_ref()
    }

    pub(crate) fn date(&self) -> Option<&DateTime<Utc>> {
        self.date.as_ref()
    }

    pub(crate) fn is_version_tag(&self) -> bool {
        self.semver.is_some()
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Error)]
pub(crate) enum TagBuilderError {
    #[error("Tag object kind identification failed")]
    TagObjectKindFailed,
    /// Error from the git2 crate
    #[error("Git2 says: {0}")]
    Git2Error(#[from] git2::Error),
    /// Error from the Semver crate
    #[error("Semver says: {0}")]
    SemverError(#[from] semver::Error),
}

#[derive(Clone)]
pub(crate) struct TagBuilder<'a> {
    id: Oid,
    repo: &'a Repository,
    name: String,
    package: Option<String>,
    semver: Option<Version>,
    date: Option<DateTime<Utc>>,
}

impl<'a> TagBuilder<'a> {
    pub(crate) fn set_package(&mut self, package: &str) -> &mut Self {
        self.package = Some(package.to_string());
        self
    }

    fn set_semver(&mut self, semver: &str) -> &mut Self {
        let semver = match Version::parse(semver) {
            Ok(sv) => Some(sv),
            Err(e) => {
                log::warn!("failed to parse `{semver:?}` to semver::Version. error: `{e}`");
                None
            }
        };
        self.semver = semver;
        self
    }

    pub(crate) fn get_semver(&mut self, release_pattern: &ReleasePattern) -> &mut Self {
        log::trace!(
            "getting semver for {} using the pattern {release_pattern:?}",
            self.name
        );
        let haystack = self.name.clone();

        match release_pattern {
            ReleasePattern::Prefix(p) => {
                log::trace!("Applying the PREFIX pattern");
                let Some(caps) = PREFIX.captures(&haystack) else {
                    log::warn!("Failed to extract captures from haystack");
                    return self;
                };
                log::trace!("Captured groups: `{caps:?}`");
                let Some(prefix) = caps.name("prefix") else {
                    log::warn!("Failed to find prefix in captures");
                    return self;
                };
                log::trace!("Prefix: `{prefix:?}`");
                let Some(semver) = caps.name("semver") else {
                    log::warn!("Failed to find semver in captures");
                    return self;
                };

                if prefix.as_str() == p.as_str() {
                    self.set_semver(semver.as_str())
                } else {
                    self
                }
            }

            ReleasePattern::PackagePrefix(p) => {
                let Some(expected_package) = self.package.as_ref() else {
                    return self;
                };

                let Some(caps) = PACKAGE_PREFIX.captures(&haystack) else {
                    return self;
                };

                let Some(prefix) = caps.name("prefix") else {
                    return self;
                };
                let Some(package) = caps.name("package") else {
                    return self;
                };
                let Some(semver) = caps.name("semver") else {
                    return self;
                };

                if prefix.as_str() == p.as_str() && package.as_str() == expected_package {
                    self.set_semver(semver.as_str())
                } else {
                    self
                }
            }
        }
    }

    pub(crate) fn get_date(&mut self) -> &mut Self {
        let Ok(git_tag) = self.repo.find_tag(self.id) else {
            return self;
        };

        let tag_object = match git_tag.peel() {
            Ok(to) => to,
            Err(_) => {
                log::warn!("could not peel {} to tag object", self.name);
                return self;
            }
        };

        let Some(kind) = tag_object.kind() else {
            log::warn!("object type not identified for {}", self.name);
            return self;
        };

        let commit;
        match kind {
            ObjectType::Commit => {
                let c = tag_object.as_commit().unwrap();
                commit = c.clone();
            }
            _ => {
                let peel = tag_object.clone().peel_to_commit();
                match peel {
                    Ok(c) => commit = c,
                    Err(e) => {
                        log::warn!("Error peeling tag to commit {e}");
                        return self;
                    }
                }
            }
        }

        let time = commit.time();
        let date = chrono::DateTime::from_timestamp(time.seconds(), 0);
        self.date = date;

        self
    }

    pub(crate) fn build(&self) -> Tag {
        Tag {
            id: self.id,
            name: self.name.clone(),
            package: self.package.clone(),
            semver: self.semver.clone(),
            date: self.date,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new_with_simple_semver() {
//         let id =
// Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/v0.1.19";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_some());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }

//     #[test]
//     fn test_new_with_pre_release_and_build_semver() {
//         let id =
// Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/v0.1.19-alpha.3+build2937";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_some());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }

//     #[test]
//     fn test_new_with_pre_release_semver() {
//         let id =
// Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/v0.1.19-alpha.3";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_some());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }

//     #[test]
//     fn test_new_with_build_semver() {
//         let id =
// Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/v0.1.19+build2937";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_some());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }

//     #[test]
//     fn test_new_with_no_semver() {
//         let id =
// Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/just my tag";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_none());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }
// }
