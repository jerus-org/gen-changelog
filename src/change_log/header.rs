use std::fmt::Display;

const DEFAULT_TITLE: &str = "Changelog";

const DEFAULT_PARAGRAPHS: [&str; 2] = [
    "All notable changes to this project will be documented in this file.",
    "The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).",
];

#[derive(Debug)]
pub(crate) struct Header {
    title: String,
    paragraphs: Vec<String>,
}

impl Default for Header {
    fn default() -> Self {
        let title = String::from(DEFAULT_TITLE);
        let mut paragraphs = Vec::new();
        for paragraph in DEFAULT_PARAGRAPHS {
            paragraphs.push(paragraph.to_string());
        }
        Self { title, paragraphs }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# {}", self.title)?;
        for para in self.paragraphs.iter() {
            writeln!(f)?;
            writeln!(f, "{para}")?;
        }
        Ok(())
    }
}
