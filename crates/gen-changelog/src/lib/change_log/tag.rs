use std::fmt::Display;

use chrono::{DateTime, Utc};
use git2::{ObjectType, Oid, Repository};
use lazy_regex::{Lazy, Regex, lazy_regex};
use semver::Version;
use thiserror::Error;

use crate::change_log_config::ReleasePattern;

// Both patterns are anchored (`^…$`) and are applied to tag names with the
// `refs/tags/` ref prefix stripped (see `TagBuilder::get_semver`). Anchoring is
// what keeps a bare `v` prefix from matching the `v<X.Y.Z>` embedded inside a
// package tag such as `mypkg-v1.2.3` (issue #274).
pub static PREFIX: Lazy<Regex> = lazy_regex!(
    r#"^(?P<prefix>\w+)(?P<semver>(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(?P<pre>-[a-z\.A-Z0-9]+)?(?P<build>\+[0-9A-Za-z-\.]+)?)$"#
);
pub static PACKAGE_PREFIX: Lazy<Regex> = lazy_regex!(
    r#"^(?P<package>(([-_]?\w+)+))-(?P<prefix>\w+)(?P<semver>(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(?P<pre>-[a-z\.A-Z0-9]+)?(?P<build>\+[0-9A-Za-z-\.]+)?)$"#
);

#[derive(Debug, Clone)]
pub(crate) struct Tag {
    id: Option<Oid>,
    name: String,
    package: Option<String>,
    semver: Option<Version>,
    date: Option<DateTime<Utc>>,
}

impl Tag {
    pub(crate) fn new(version: &str) -> Tag {
        let semver = Version::parse(version);
        let date = chrono::Utc::now();

        match semver {
            Ok(_) => {
                let semver = Some(semver.unwrap());
                let date = Some(date);
                Tag {
                    id: None,
                    name: version.to_string(),
                    package: None,
                    semver,
                    date,
                }
            }
            Err(_) => Tag {
                id: None,
                name: "Unreleased".to_string(),
                package: None,
                semver: None,
                date: None,
            },
        }
    }

    pub(crate) fn builder<S: Display>(
        id: Option<Oid>,
        name: S,
        repo: &Repository,
    ) -> TagBuilder<'_> {
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

    pub(crate) fn id(&self) -> Option<&Oid> {
        self.id.as_ref()
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
    id: Option<Oid>,
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
        // Match against the short tag name; `tag_foreach` yields full ref names
        // like `refs/tags/v1.2.3`, and the anchored patterns expect the tag name
        // to start at the prefix (or package) segment.
        let haystack = self.name.trim_start_matches("refs/tags/").to_string();

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
        if self.id.is_none() {
            let date = Some(chrono::Utc::now());
            self.date = date;
            return self;
        }

        let Ok(git_tag) = self.repo.find_tag(self.id.unwrap()) else {
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

#[cfg(test)]
mod regex_tests {
    //! Anchoring contract for the tag-matching regexes (issue #274).
    //!
    //! Names here are already stripped of the `refs/tags/` ref prefix, matching
    //! what [`TagBuilder::get_semver`] feeds the regexes after trimming.
    use super::{PACKAGE_PREFIX, PREFIX};

    #[test]
    fn prefix_matches_bare_version() {
        let caps = PREFIX.captures("v0.1.2").expect("should match bare v tag");
        assert_eq!(caps.name("prefix").unwrap().as_str(), "v");
        assert_eq!(caps.name("semver").unwrap().as_str(), "0.1.2");
    }

    #[test]
    fn prefix_matches_pre_release() {
        let caps = PREFIX
            .captures("v1.0.0-rc.1")
            .expect("should match pre-release");
        assert_eq!(caps.name("prefix").unwrap().as_str(), "v");
        assert_eq!(caps.name("semver").unwrap().as_str(), "1.0.0-rc.1");
    }

    #[test]
    fn prefix_rejects_package_tag() {
        // The core of #274: a bare `v` prefix must NOT match the `v0.1.2`
        // embedded inside a crate tag.
        assert!(
            PREFIX.captures("gen-circleci-orb-v0.1.2").is_none(),
            "bare-v PREFIX must not match a package-prefixed tag"
        );
    }

    #[test]
    fn package_prefix_matches_crate_tag() {
        let caps = PACKAGE_PREFIX
            .captures("gen-circleci-orb-v0.1.2")
            .expect("should match crate tag");
        assert_eq!(caps.name("package").unwrap().as_str(), "gen-circleci-orb");
        assert_eq!(caps.name("prefix").unwrap().as_str(), "v");
        assert_eq!(caps.name("semver").unwrap().as_str(), "0.1.2");
    }

    #[test]
    fn package_prefix_rejects_bare_version() {
        assert!(
            PACKAGE_PREFIX.captures("v0.1.2").is_none(),
            "package-prefix pattern must not match a bare workspace tag"
        );
    }
}
