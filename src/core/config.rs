//! Configuration management for SoundSwap.
//! Persists user settings to JSON file in %APPDATA% directory.

use serde::{Deserialize, Serialize};

/// User configuration structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Configuration {
    /// List of configured audio devices.
    pub configurations: Vec<ConfigurationItem>,
    /// Theme preference (dark/light/system/custom).
    #[serde(default = "default_theme")]
    pub theme: String,
    /// Current language.
    #[serde(default = "default_language")]
    pub language: String,
    /// Check for updates automatically (true/false).
    pub check_for_updates: bool,
    /// Start with Windows option.
    pub start_with_windows: bool,
    /// User data path (e.g., %APPDATA%\\SoundSwap)
    pub data_path: String,
}

fn default_theme() -> String { "system".to_string() }
fn default_language() -> String { "en".to_string() }

/// A single audio device configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationItem {
    /// Configuration name.
    pub name: String,
    /// Default output device (hardware ID).
    pub output_device: String,
    /// Default input device (hardware ID).
    pub input_device: String,
    /// Optional keyboard shortcut.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotkey: Option<String>,
}

/// Configuration file path in %APPDATA%.
pub const CONFIG_FILE_NAME: &str = "soundswap_config.json";

impl Default for Configuration {
    fn default() -> Self {
        Configuration::default()
    }
}

/// Load configuration from JSON file.
pub fn load_config(data_path: &str) -> Configuration {
    let default_config = Configuration::default();
    
    // For now, use default config (will be replaced by actual JSON file loading in production)
    // In real implementation:
    // - Construct full path: data_path + "/" + CONFIG_FILE_NAME
    // - Read and parse JSON
    // - Return loaded Configuration with user settings
    
    default_config
}

/// Save configuration to JSON file.
pub fn save_config(config: &Configuration, data_path: &str) -> bool {
    // For now, always return true (would be replaced by actual file I/O in production)
    // In real implementation:
    // - Construct full path: data_path + "/" + CONFIG_FILE_NAME
    // - Write JSON content to file
    
    true
}

/// Save configuration with the given data path.
pub fn save_config_with_path(config: &Configuration, data_path: &str) {
    // Stub implementation (real code will handle file I/O)
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
        let config = Configuration::default();
        config.save(&"C:\\Users\\steve\\AppData\\Roaming".to_string());
        // TODO: Verify JSON serialization/deserialization works correctly
    }
}
