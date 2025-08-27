use std::fmt::Display;

use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub(crate) enum LinkError {
    /// Parse error reported by Url crate
    #[error("Url says parse failed because: {0}")]
    UrlError(#[from] url::ParseError),
}

#[derive(Debug)]
pub(crate) struct Link {
    anchor: String,
    url: Url,
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}] {}", self.anchor, self.url)
    }
}

impl Link {
    pub(crate) fn new(anchor: &str, url: &str) -> Result<Self, LinkError> {
        let anchor = anchor.to_string();
        let url = Url::parse(url)?;

        Ok(Link { anchor, url })
    }
}
