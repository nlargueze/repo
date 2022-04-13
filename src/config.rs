//! Configuration

use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Configuration directory
pub const CONFIG_DIR: &str = ".gitt";

/// Configuration file name
pub const CONFIG_FILE: &str = "config.toml";

/// Commits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigCommits {
    /// Commit types (key + description)
    pub types: BTreeMap<String, String>,
}

impl Default for ConfigCommits {
    fn default() -> Self {
        let mut types = BTreeMap::new();
        types.insert("feat".to_string(), "A new feature".to_string());
        types.insert("fix".to_string(), "Bug fixes".to_string());
        types.insert("docs".to_string(), "Documentation".to_string());
        types.insert("style".to_string(), "Code styling".to_string());
        types.insert("refactor".to_string(), "Code refactoring".to_string());
        types.insert("perf".to_string(), "Performance Improvements".to_string());
        types.insert("test".to_string(), "Tests".to_string());
        types.insert("build".to_string(), "Build system".to_string());
        types.insert("ci".to_string(), "Continuous Integration".to_string());
        types.insert("cd".to_string(), "Continuous Delivery".to_string());
        types.insert("chore".to_string(), "Other changes".to_string());

        Self { types }
    }
}

/// Changelog configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigChangeLog {}

/// Configuration object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Commits config
    pub commits: ConfigCommits,
    /// Changelog config
    pub changelog: ConfigChangeLog,
}

impl Config {
    /// Loads the configuration file from the repo
    pub fn load(repo_path: &PathBuf) -> Result<Self> {
        let file = repo_path.join(CONFIG_DIR).join(CONFIG_FILE);
        let cfg_str = fs::read_to_string(file)?;
        Ok(toml::from_str::<Config>(&cfg_str)?)
    }

    /// Saves a [Configuration] to the repo
    pub fn save(&self, repo_path: &PathBuf) -> Result<()> {
        let cfg_str = toml::to_string(self)?;
        if !repo_path.join(CONFIG_DIR).exists() {
            fs::create_dir(repo_path.join(CONFIG_DIR))?;
        }
        fs::write(repo_path.join(CONFIG_DIR).join(CONFIG_FILE), cfg_str)?;
        Ok(())
    }

    /// Checks if a repo is already initialized
    pub fn is_initialized(repo_path: &PathBuf) -> bool {
        repo_path.join(CONFIG_DIR).join(CONFIG_FILE).exists()
    }

    /// Returns a list of valid types
    pub fn valid_types(&self) -> Vec<String> {
        self.commits.types.keys().map(|s| s.clone()).collect()
    }
}
