/// SoundSwap Core Library - Windows audio device switching and application services.
/// Provides configuration management, theme support, and update checking.

pub mod audio_service;
pub mod config;
pub mod theme;
pub mod updater;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_modules_load() {
        let config = config::load_config("C:\\Users\\steve\\AppData\\Roaming");
        assert_eq!(config.theme, "system");

        // This is a hardware-dependent query. The test only verifies that the
        // service can be called; the CI runner is not required to expose audio
        // endpoints.
        let _audio_devices = audio_service::list_audio_devices();
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
        use super::theme::Theme;

        let dark = Theme::Dark;
        assert_eq!(dark.get_color_mode(), "dark");

        let light = Theme::Light;
        assert_eq!(light.get_color_mode(), "light");

        let system = Theme::System;
        assert_eq!(system.get_color_mode(), "dark");
    }

    #[test]
    fn test_update_options() {
        use super::updater::UpdateOptions;

        let options = UpdateOptions::default();
        assert_eq!(options.owner, "Stevehunn");
        assert_eq!(options.repository, "soundswaper");
        assert_eq!(options.check_interval_seconds, 43200);
    }
}
