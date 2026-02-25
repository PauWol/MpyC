use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

use crate::util::path::{config_file_exists, get_config_file, make_config_file};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub active_version: String,
}

impl Config {
    /// Load config from file, auto-creating default if missing or invalid
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = get_config_file();

        // If file doesn't exist, create it and return default
        if !config_file_exists() {
            make_config_file()?;
            return Ok(Config {
                active_version: String::new(),
            });
        }

        // Read file
        let content = fs::read_to_string(&path)?;

        // If empty file, return default
        if content.trim().is_empty() {
            return Ok(Config {
                active_version: String::new(),
            });
        }

        // Try to parse
        let config: Config = toml::from_str(&content)?;

        // Validate; if invalid, return default
        if !config.is_valid() {
            eprintln!("Warning: config invalid, resetting to default.");
            return Ok(Config {
                active_version: String::new(),
            });
        }

        Ok(config)
    }

    /// Save config to file
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = get_config_file();
        let toml_str = toml::to_string_pretty(self)?;
        fs::write(path, toml_str)?;
        Ok(())
    }

    /// Check if config is valid
    pub fn is_valid(&self) -> bool {
        let version = self.active_version.trim();

        if version.is_empty() {
            return false;
        }

        if !version
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        {
            return false;
        }

        if version.starts_with('.')
            || version.ends_with('.')
            || version.starts_with('-')
            || version.ends_with('-')
        {
            return false;
        }

        if version.len() > 50 {
            return false;
        }

        true
    }
}
