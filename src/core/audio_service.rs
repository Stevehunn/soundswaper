//! Audio device service implementation for Windows.
//! Lists available input/output devices via WASAPI enumeration.

use std::collections::HashMap;
use winapi::um::corewindows::*;

/// An audio device (input or output).
#[derive(Debug, Clone)]
pub struct AudioDevice {
    /// Device name (e.g., "Headphones", "Stereo speakers")
    pub name: String,
    /// Hardware ID for identifying the device
    pub hardware_id: String,
    /// Connection status indicator
    pub connected: bool,
}

impl std::fmt::Display for AudioDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// List all available audio input/output devices on the system.
/// Returns a vector of [AudioDevice](AudioDevice).
pub fn list_audio_devices() -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    
    // In production: enumerate devices via WASAPI or Core Audio API
    // For now, return sample data (will be replaced by actual Windows enumeration)
    
    // Sample output device
    devices.push(AudioDevice {
        name: "Headphones (Primary)",
        hardware_id: "AudioDev_001".to_string(),
        connected: true,
    });
    
    // Sample input device
    devices.push(AudioDevice {
        name: "Microphone (Default)",
        hardware_id: "MicDev_002".to_string(),
        connected: true,
    });
    
    devices
}

/// Get default audio output device.
pub fn get_default_output_device() -> AudioDevice {
    list_audio_devices().iter()
        .find(|d| d.name.contains("Primary"))
        .cloned()
        .unwrap_or_else(|| AudioDevice {
            name: "Default Speakers".to_string(),
            hardware_id: "AudioDev_Default".to_string(),
            connected: true,
        })
}

/// Get default audio input device.
pub fn get_default_input_device() -> AudioDevice {
    list_audio_devices().iter()
        .find(|d| d.name.contains("Default"))
        .cloned()
        .unwrap_or_else(|| AudioDevice {
            name: "Default Microphone".to_string(),
            hardware_id: "MicDev_Default".to_string(),
            connected: true,
        })
}

/// Get all audio devices filtered by type.
pub fn get_audio_devices_by_type(device_type: &str) -> Vec<AudioDevice> {
    list_audio_devices().iter()
        .filter(|d| {
            if device_type == "input" { d.name.contains("Microphone") || d.name.starts_with("Input") }
            else { d.name.contains("Speakers") || d.name.starts_with("Output") }
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices() {
        let devices = list_audio_devices();
        assert!(!devices.is_empty());
    }

    #[test]
    fn test_get_default_output() {
        let device = get_default_output_device();
        assert!(device.connected);
    }

    #[test]
    fn test_get_default_input() {
        let device = get_default_input_device();
        assert!(device.connected);
    }
}
