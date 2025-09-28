#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]
#![doc = include_str!("../docs/lib.md")]

mod change_log;
mod change_log_config;
mod error;
#[cfg(test)]
pub(crate) mod test_utils;

pub use change_log::DEFAULT_CHANGELOG_FILENAME;
pub use change_log::{ChangeLog, ChangeLogBuilder};
pub use change_log_config::ChangeLogConfig;
pub use error::Error;
