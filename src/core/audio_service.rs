//! Audio device service interface.
//!
//! Actual Windows Core Audio/WASAPI enumeration and device switching are not
//! implemented yet. Until that backend is added, this module deliberately
//! returns no enumerated devices instead of presenting fabricated devices.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDevice {
    pub name: String,
    pub hardware_id: String,
    pub connected: bool,
}

impl std::fmt::Display for AudioDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// List available audio input/output devices.
///
/// This currently returns an empty list; it does not pretend that sample
/// devices are connected to the user's computer.
pub fn list_audio_devices() -> Vec<AudioDevice> {
    Vec::new()
}

/// Return a disconnected placeholder until the Windows audio backend exists.
pub fn get_default_output_device() -> AudioDevice {
    unavailable_device("Default output device")
}

/// Return a disconnected placeholder until the Windows audio backend exists.
pub fn get_default_input_device() -> AudioDevice {
    unavailable_device("Default input device")
}

fn unavailable_device(name: &str) -> AudioDevice {
    AudioDevice {
        name: name.to_string(),
        hardware_id: String::new(),
        connected: false,
    }
}

/// Filter enumerated devices by "input" or "output".
pub fn get_audio_devices_by_type(device_type: &str) -> Vec<AudioDevice> {
    list_audio_devices()
        .into_iter()
        .filter(|device| match device_type {
            "input" => device.name.to_lowercase().contains("microphone")
                || device.name.to_lowercase().starts_with("input"),
            "output" => device.name.to_lowercase().contains("speaker")
                || device.name.to_lowercase().starts_with("output")
                || device.name.to_lowercase().contains("headphone"),
            _ => false,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices_does_not_fabricate_hardware() {
        assert!(list_audio_devices().is_empty());
    }

    #[test]
    fn test_default_devices_are_marked_unavailable() {
        assert!(!get_default_output_device().connected);
        assert!(!get_default_input_device().connected);
    }

    #[test]
    fn test_unknown_device_type_returns_no_devices() {
        assert!(get_audio_devices_by_type("unknown").is_empty());
    }
}
