use std::fmt::Display;

use chrono::{DateTime, Utc};
use git2::{Oid, Repository};
use semver::Version;

use lazy_regex::{Lazy, Regex, lazy_regex};

pub static SEMVER: Lazy<Regex> = lazy_regex!(
    r#"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(?P<pre>-[a-z\.A-Z0-9]+)?(?P<build>\+[0-9A-Za-z-\.]+)?"#
);

#[derive(Debug, Clone)]
pub(crate) struct Tag {
    id: Oid,
    name: String,
    semver: Option<Version>,
    date: Option<DateTime<Utc>>,
}

impl Tag {
    pub(crate) fn new<S: Display>(id: Oid, name: S, repo: &Repository) -> Self {
        let name = name.to_string();
        let semver = if let Some(v) = SEMVER.captures(&name) {
            Version::parse(v.get(0).unwrap().as_str()).ok()
        } else {
            None
        };

        let date = if semver.is_some() {
            Self::get_date(&id, repo)
        } else {
            None
        };

        Tag {
            id,
            name,
            semver,
            date,
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

    pub(crate) fn get_date(id: &Oid, repo: &Repository) -> Option<DateTime<Utc>> {
        let Ok(git_tag) = repo.find_tag(*id) else {
            return None;
        };

        let tagged_id = git_tag.target_id();
        let Ok(commit) = repo.find_commit(tagged_id) else {
            return None;
        };

        let time = commit.time();
        chrono::DateTime::from_timestamp(time.seconds(), 0)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new_with_simple_semver() {
//         let id = Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/v0.1.19";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_some());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }

//     #[test]
//     fn test_new_with_pre_release_and_build_semver() {
//         let id = Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/v0.1.19-alpha.3+build2937";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_some());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }

//     #[test]
//     fn test_new_with_pre_release_semver() {
//         let id = Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/v0.1.19-alpha.3";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_some());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }

//     #[test]
//     fn test_new_with_build_semver() {
//         let id = Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/v0.1.19+build2937";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_some());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }

//     #[test]
//     fn test_new_with_no_semver() {
//         let id = Oid::from_str("00d7d05408751ccf747371e6b1f2b142a8314fbd").unwrap();
//         let name = "refs/tags/just my tag";

//         let tag = Tag::new(id, name);

//         assert!(tag.semver.is_none());
//         assert_eq!(tag.name, name);
//         assert_eq!(tag.id, id);
//     }
// }
