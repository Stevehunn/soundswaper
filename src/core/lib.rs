/// SoundSwap Core Library - Audio device switching with Windows API integration
/// Provides configuration management, theme system, and update checking functionality.

pub mod audio_service;
pub mod config;
pub mod theme;
pub mod updater;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_modules_load() {
        // Test that all core modules can be imported and initialized
        let config = config::load_config("C:\\Users\\steve\\AppData\\Roaming");
        assert_eq!(config.theme, "system");
        
        let audio_devices = audio_service::list_audio_devices();
        assert!(audio_devices.is_empty()); // WASAPI enumeration is not implemented yet
    }

    #[test]
    fn test_default_configuration() {
        let config = config::Configuration::default();
        assert_eq!(config.theme, "system");
        assert_eq!(config.language, "en");
        assert!(config.check_for_updates);
        assert!(config.start_with_windows);
    }

    #[test]
    fn test_theme_modes() {
        use super::theme::{Theme, is_light_theme};
        
        // Test dark theme
        let dark = Theme::Dark;
        assert_eq!(dark.get_color_mode(), "dark");
        
        // Test light theme
        let light = Theme::Light;
        assert_eq!(light.get_color_mode(), "light");
        
        // Test system theme (assumes dark by default)
        let system = Theme::System;
        assert_eq!(system.get_color_mode(), "dark");
    }

    #[test]
    fn test_update_options() {
        use super::updater::UpdateOptions;
        
        let options = UpdateOptions::default();
        assert_eq!(options.owner, "Stevehunn");
        assert_eq!(options.repository, "soundswaper");
        assert_eq!(options.check_interval_seconds, 43200); // 12 hours
    }
}
