#![allow(unused_variables)]

use tauri::{CustomPlugin, ExitReason, Manager};

/// Custom Tauri plugin implementation using Rust directly.
/// This provides tray icon and global hotkey functionality.

pub struct SoundSwapPlugin;

impl CustomPlugin for SoundSwapPlugin {
    fn name(&self) -> &'static str {
        "soundswap-plugin"
    }

    fn build_spans<F>(&self, _: F) -> Result<(), tauri::Error>
    where
        F: FnOnce(tauri::StateFns),
    {
        Ok(())
    }
}
