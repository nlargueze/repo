//! Wrappers for `git tag`

use std::fmt::Display;

use chrono::{DateTime, Utc};
use semver::Version;

use crate::error::Result;

/// Git tag
#[derive(Debug, Clone, Eq, Ord)]
pub struct GitTag {
    /// Tag name
    pub tag: String,
    /// Tag commmit hash
    pub hash: String,
    /// Tag date
    pub date: DateTime<Utc>,
    /// Tag message
    pub message: Option<String>,
}

impl PartialEq for GitTag {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl PartialOrd for GitTag {
    fn partial_cmp(&self, other: &GitTag) -> Option<std::cmp::Ordering> {
        Some(self.tag.cmp(&other.tag))
    }
}

impl Display for GitTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.tag,
            self.message
                .clone()
                .map(|m| format!(" | {}", m))
                .unwrap_or_default()
        )
    }
}

impl GitTag {
    /// Returns the semver version for a tag
    pub fn version(&self) -> Result<Version> {
        Ok(Version::parse(&self.tag)?)
    }
}
