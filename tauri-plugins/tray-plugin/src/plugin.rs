//! Tray notification plugin for SoundSwap.
//! Handles notification area integration (tray icon, context menu).

use tauri::{
    plugins::manager::State,
    Event,
    EventManager,
    WindowLabel,
};
use std::sync::OnceLock;

/// Tray icon label reference.
pub const TRAY_LABEL: &str = "window:tray";

/// Tray plugin state (holds current configuration).
#[derive(Clone)]
pub struct TrayState {
    /// Current audio configuration ID.
    pub config_id: u32,
}

impl Default for TrayState {
    fn default() -> Self {
        TrayState {
            config_id: 1, // First configuration by default
        }
    }
}

/// Plugin handler for tray icon events.
pub struct TrayHandler {
    /// Window label (tray).
    window_label: WindowLabel,
    /// App state (configuration data).
    pub app_state: OnceLock<TrayState>,
}

impl TrayHandler {
    /// Initialize the plugin with a window label reference and app state.
    pub fn new(window_label: impl Into<WindowLabel>, app_state: OnceLock<TrayState>) -> Self {
        TrayHandler {
            window_label: window_label.into(),
            app_state,
        }
    }

    /// Initialize the plugin (stub implementation).
    pub async fn init(&self) {}

    /// Handle left-click event on tray icon (opens main window).
    pub async fn handle_click(&self) -> Event {
        // Left click: show or minimize main window
        unimplemented!()
    }

    /// Handle right-click context menu on tray icon.
    pub async fn handle_context_menu(&self, menu_id: u32) -> Event {
        // Right click: switch audio configuration (stub)
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray_handler_new() {
        let app_state = OnceLock::new();
        let handler = TrayHandler::new("window:tray", app_state);
        
        assert_eq!(handler.app_state.get(), None);
    }
}
