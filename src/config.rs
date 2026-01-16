use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Update interval in milliseconds
    pub update_interval: u64,
    /// Disable colors
    pub no_color: bool,
    /// Default sort column (pid, name, cpu, memory)
    pub sort_by: String,
    /// Sort ascending
    pub sort_ascending: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update_interval: 250,
            no_color: false,
            sort_by: "cpu".to_string(),
            sort_ascending: false,
        }
    }
}

impl Config {
    /// Get the config file path (~/.config/mprobe/config.toml)
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("mprobe").join("config.toml"))
    }

    /// Load config from file, returning default if file doesn't exist
    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Warning: Failed to parse config file: {}", e);
                    Self::default()
                }
            },
            Err(e) => {
                eprintln!("Warning: Failed to read config file: {}", e);
                Self::default()
            }
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Could not determine config directory")?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = toml::to_string_pretty(self).map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, content).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
}
