//! Configuration management for SoundSwap.
//! Persists user settings to a JSON file in the user's application data directory.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Configuration {
    pub configurations: Vec<ConfigurationItem>,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_language")]
    pub language: String,
    pub check_for_updates: bool,
    pub start_with_windows: bool,
    pub data_path: String,
}

fn default_theme() -> String { "system".to_string() }
fn default_language() -> String { "en".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationItem {
    pub name: String,
    pub output_device: String,
    pub input_device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotkey: Option<String>,
}

pub const CONFIG_FILE_NAME: &str = "soundswap_config.json";

impl Default for Configuration {
    fn default() -> Self {
        Self {
            configurations: Vec::new(),
            theme: default_theme(),
            language: default_language(),
            check_for_updates: true,
            start_with_windows: true,
            data_path: String::new(),
        }
    }
}

impl Configuration {
    /// Save this configuration in the supplied directory.
    pub fn save(&self, data_path: &str) -> std::io::Result<()> {
        save_config_to_path(self, data_path)
    }
}

fn config_file_path(data_path: &str) -> PathBuf {
    Path::new(data_path).join(CONFIG_FILE_NAME)
}

/// Load configuration from disk, falling back to defaults if it is absent or invalid.
pub fn load_config(data_path: &str) -> Configuration {
    let path = config_file_path(data_path);
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<Configuration>(&contents).ok())
        .map(|mut config| {
            if config.data_path.is_empty() {
                config.data_path = data_path.to_string();
            }
            config
        })
        .unwrap_or_else(|| Configuration {
            data_path: data_path.to_string(),
            ..Configuration::default()
        })
}

/// Save configuration as JSON. Returns false if the directory or file cannot be written.
pub fn save_config(config: &Configuration, data_path: &str) -> bool {
    save_config_to_path(config, data_path).is_ok()
}

/// Save configuration and propagate filesystem errors.
pub fn save_config_to_path(config: &Configuration, data_path: &str) -> std::io::Result<()> {
    fs::create_dir_all(data_path)?;
    let contents = serde_json::to_vec_pretty(config)
        .map_err(std::io::Error::other)?;
    fs::write(config_file_path(data_path), contents)
}

/// Backwards-compatible helper; errors are intentionally ignored by this legacy void API.
pub fn save_config_with_path(config: &Configuration, data_path: &str) {
    let _ = save_config(config, data_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Configuration::default();
        assert_eq!(config.theme, "system");
        assert_eq!(config.language, "en");
        assert!(config.check_for_updates);
        assert!(config.start_with_windows);
    }

    #[test]
    fn test_load_save_config() {
        let directory = std::env::temp_dir().join(format!("soundswap-config-{}", std::process::id()));
        let path = directory.to_string_lossy().to_string();
        let mut config = Configuration::default();
        config.language = "fr".to_string();
        config.save(&path).expect("configuration should save");
        let loaded = load_config(&path);
        assert_eq!(loaded.language, "fr");
        let _ = fs::remove_dir_all(directory);
    }
}
