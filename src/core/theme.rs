//! Theme system with support for dark, light, system, and custom themes.

use serde::{Deserialize, Serialize};

/// Available theme presets.
#[derive(Debug, Clone, Default)]
pub struct Presets {
    pub name: String,
    pub colors: CustomThemeColors,
}

/// Color values for a custom theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeColors {
    /// Background color (hex string).
    pub background: String,
    /// Cards/content area color (hex string).
    pub cards: String,
    /// Text color (hex string).
    pub text: String,
    /// Accent color for buttons/highlights (hex string).
    pub accent: String,
}

/// Theme configuration (dark/light/system/custom).
#[derive(Debug, Clone)]
pub enum Theme {
    Dark,
    Light,
    System,
    Custom(CustomThemeColors),
}

impl Default for Theme {
    fn default() -> Self {
        Theme::System
    }
}

/// Check if the system prefers a light theme.
pub fn is_light_theme() -> bool {
    // In real implementation: check Windows registry or environment variable
    // For now, return false (assume dark theme)
    false
}

impl Theme {
    /// Apply the theme to the application.
    pub fn apply(&self) {
        match self {
            Theme::Dark => {
                // Apply dark theme CSS classes
            }
            Theme::Light => {
                // Apply light theme CSS classes
            }
            Theme::System => {
                if is_light_theme() {
                    // Apply light theme
                } else {
                    // Apply dark theme
                }
            }
            Theme::Custom(ref colors) => {
                // Apply custom theme colors
            }
        }
    }

    /// Get current theme from configuration.
    pub fn from_config() -> Self {
        // Load theme from JSON config file
        // For now, return System as default
        Theme::System
    }

    /// Get the color mode (dark/light) for CSS application.
    pub fn get_color_mode(&self) -> &'static str {
        match self {
            Theme::Dark | Theme::Custom(_) => "dark",
            Theme::Light => "light",
            Theme::System => {
                if is_light_theme() { "light" } else { "dark" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme, Theme::System);
    }

    #[test]
    fn test_dark_mode() {
        let theme = Theme::Dark;
        assert_eq!(theme.get_color_mode(), "dark");
    }

    #[test]
    fn test_light_mode() {
        let theme = Theme::Light;
        assert_eq!(theme.get_color_mode(), "light");
    }

    #[test]
    fn test_custom_theme() {
        let colors = CustomThemeColors {
            background: "#1a1a2e".to_string(),
            cards: "#16213e".to_string(),
            text: "#eaeaea".to_string(),
            accent: "#0f3460".to_string(),
        };
        let theme = Theme::Custom(colors.clone());
        assert_eq!(theme.get_color_mode(), "dark"); // Custom themes default to dark
    }

    #[test]
    fn test_system_theme() {
        let theme = Theme::System;
        assert_eq!(theme.get_color_mode(), "dark"); // Default assumption (will change based on OS)
    }
}
