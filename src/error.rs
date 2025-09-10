use thiserror::Error;

/// Error messages for the gen-changelog crate
#[derive(Debug, Error)]
pub enum Error {
    /// url not found
    #[error("url not found")]
    UrlNotFound,
    /// capture groups not found
    #[error("capture groups not found")]
    CapturesNotFound,
    /// owner not found in capture group
    #[error("owner not found in capture group")]
    OwnerNotFound,
    /// repo not found in capture group
    #[error("repo not found in capture group")]
    RepoNotFound,
    /// Error from the git2 crate
    #[error("Git2 says: {0}")]
    Git2Error(#[from] git2::Error),
    /// Error from the std io
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    /// Error from the toml serializer
    #[error("toml serializer error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    /// Error from the toml serializer
    #[error("toml deserializer error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    // /// Error from the regex crate
    // #[error("Regex says: {0}")]
    // RegexError(#[from] regex::Error),
}
