//! Windows audio endpoint discovery and default-device switching.
//!
//! Device enumeration uses the Windows Core Audio MMDevice API through the
//! `wasapi` crate. Changing the system default endpoint uses Microsoft's
//! undocumented `IPolicyConfig` COM interface (via `com-policy-config`).
//! The latter is not a public Windows contract and may need adjustment after
//! future Windows updates.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDevice {
    /// Friendly name shown to the user.
    pub name: String,
    /// Opaque Windows endpoint ID. Persist this value; do not derive it from the name.
    pub hardware_id: String,
    /// Whether Windows currently reports this endpoint as connected/active.
    pub connected: bool,
}

impl fmt::Display for AudioDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub type AudioServiceResult<T> = Result<T, String>;

fn unavailable_device(name: &str) -> AudioDevice {
    AudioDevice {
        name: name.to_string(),
        hardware_id: String::new(),
        connected: false,
    }
}

/// Enumerate active playback and capture endpoints.
///
/// Prefer this error-aware API in application code. It distinguishes an
/// actual empty device list from a COM or Windows audio-service failure.
#[cfg(windows)]
pub fn try_list_audio_devices() -> AudioServiceResult<Vec<AudioDevice>> {
    with_com_apartment(|| {
        let enumerator = wasapi::DeviceEnumerator::new().map_err(|error| error.to_string())?;
        let mut devices = enumerate_direction(&enumerator, wasapi::Direction::Render)?;
        devices.extend(enumerate_direction(
            &enumerator,
            wasapi::Direction::Capture,
        )?);
        Ok(devices)
    })
}

/// Non-Windows builds do not have a Windows Core Audio backend.
#[cfg(not(windows))]
pub fn try_list_audio_devices() -> AudioServiceResult<Vec<AudioDevice>> {
    Err("Windows audio device management is only available on Windows".to_string())
}

/// Compatibility wrapper for callers that cannot yet handle errors.
///
/// New UI and command code should call `try_list_audio_devices` so that an
/// unavailable Windows audio service is not mistaken for an empty device list.
pub fn list_audio_devices() -> Vec<AudioDevice> {
    try_list_audio_devices().unwrap_or_default()
}

/// Enumerate active endpoints of one direction: `input` or `output`.
#[cfg(windows)]
pub fn try_get_audio_devices_by_type(device_type: &str) -> AudioServiceResult<Vec<AudioDevice>> {
    let direction = match device_type {
        "input" => wasapi::Direction::Capture,
        "output" => wasapi::Direction::Render,
        _ => return Ok(Vec::new()),
    };

    with_com_apartment(move || {
        let enumerator = wasapi::DeviceEnumerator::new().map_err(|error| error.to_string())?;
        enumerate_direction(&enumerator, direction)
    })
}

#[cfg(not(windows))]
pub fn try_get_audio_devices_by_type(_device_type: &str) -> AudioServiceResult<Vec<AudioDevice>> {
    Err("Windows audio device management is only available on Windows".to_string())
}

/// Compatibility wrapper; prefer `try_get_audio_devices_by_type`.
pub fn get_audio_devices_by_type(device_type: &str) -> Vec<AudioDevice> {
    try_get_audio_devices_by_type(device_type).unwrap_or_default()
}

/// Read the current default playback endpoint for the Console role.
#[cfg(windows)]
pub fn try_get_default_output_device() -> AudioServiceResult<AudioDevice> {
    try_get_default_device(wasapi::Direction::Render)
}

#[cfg(not(windows))]
pub fn try_get_default_output_device() -> AudioServiceResult<AudioDevice> {
    Err("Windows audio device management is only available on Windows".to_string())
}

/// Read the current default capture endpoint for the Console role.
#[cfg(windows)]
pub fn try_get_default_input_device() -> AudioServiceResult<AudioDevice> {
    try_get_default_device(wasapi::Direction::Capture)
}

#[cfg(not(windows))]
pub fn try_get_default_input_device() -> AudioServiceResult<AudioDevice> {
    Err("Windows audio device management is only available on Windows".to_string())
}

/// Compatibility wrapper; prefer `try_get_default_output_device`.
pub fn get_default_output_device() -> AudioDevice {
    try_get_default_output_device()
        .unwrap_or_else(|_| unavailable_device("Default output device (unavailable)"))
}

/// Compatibility wrapper; prefer `try_get_default_input_device`.
pub fn get_default_input_device() -> AudioDevice {
    try_get_default_input_device()
        .unwrap_or_else(|_| unavailable_device("Default input device (unavailable)"))
}

/// Set the default Windows playback endpoint.
#[cfg(windows)]
pub fn set_default_output_device(device_id: &str) -> AudioServiceResult<()> {
    let device_id = device_id.to_string();
    with_com_apartment(move || {
        set_default_device(&device_id, wasapi::Direction::Render)
    })
}

#[cfg(not(windows))]
pub fn set_default_output_device(_device_id: &str) -> AudioServiceResult<()> {
    Err("Windows audio device management is only available on Windows".to_string())
}

/// Set the default Windows capture endpoint.
#[cfg(windows)]
pub fn set_default_input_device(device_id: &str) -> AudioServiceResult<()> {
    let device_id = device_id.to_string();
    with_com_apartment(move || {
        set_default_device(&device_id, wasapi::Direction::Capture)
    })
}

#[cfg(not(windows))]
pub fn set_default_input_device(_device_id: &str) -> AudioServiceResult<()> {
    Err("Windows audio device management is only available on Windows".to_string())
}

#[cfg(windows)]
fn with_com_apartment<T, F>(operation: F) -> AudioServiceResult<T>
where
    T: Send + 'static,
    F: FnOnce() -> AudioServiceResult<T> + Send + 'static,
{
    // Tauri may call this service from a thread whose COM apartment is already
    // initialized as STA. A dedicated MTA thread avoids RPC_E_CHANGED_MODE and
    // keeps every WASAPI COM object on the apartment where it was created.
    std::thread::Builder::new()
        .name("soundswap-audio-com".to_string())
        .spawn(move || {
            wasapi::initialize_mta()
                .ok()
                .map_err(|error| format!("Could not initialize Windows COM: {error}"))?;

            struct ComApartmentGuard;
            impl Drop for ComApartmentGuard {
                fn drop(&mut self) {
                    wasapi::deinitialize();
                }
            }

            let _guard = ComApartmentGuard;
            operation()
        })
        .map_err(|error| format!("Could not start audio worker thread: {error}"))?
        .join()
        .map_err(|_| "The Windows audio worker thread panicked".to_string())?
}

#[cfg(windows)]
fn enumerate_direction(
    enumerator: &wasapi::DeviceEnumerator,
    direction: wasapi::Direction,
) -> AudioServiceResult<Vec<AudioDevice>> {
    let collection = enumerator
        .get_device_collection(&direction)
        .map_err(|error| format!("Could not enumerate {direction} devices: {error}"))?;
    let count = collection
        .get_nbr_devices()
        .map_err(|error| format!("Could not read the {direction} device count: {error}"))?;

    let mut devices = Vec::with_capacity(count as usize);
    for index in 0..count {
        let device = collection.get_device_at_index(index).map_err(|error| {
            format!("Could not read {direction} device at index {index}: {error}")
        })?;
        let hardware_id = device
            .get_id()
            .map_err(|error| format!("Could not read a {direction} device ID: {error}"))?;
        let name = device
            .get_friendlyname()
            .unwrap_or_else(|_| hardware_id.clone());

        devices.push(AudioDevice {
            name,
            hardware_id,
            connected: true,
        });
    }

    Ok(devices)
}

#[cfg(windows)]
fn try_get_default_device(direction: wasapi::Direction) -> AudioServiceResult<AudioDevice> {
    with_com_apartment(move || {
        let enumerator = wasapi::DeviceEnumerator::new().map_err(|error| error.to_string())?;
        let device = enumerator
            .get_default_device(&direction)
            .map_err(|error| format!("Could not read the default {direction} device: {error}"))?;
        let hardware_id = device
            .get_id()
            .map_err(|error| format!("Could not read the default {direction} device ID: {error}"))?;
        let name = device
            .get_friendlyname()
            .unwrap_or_else(|_| hardware_id.clone());

        Ok(AudioDevice {
            name,
            hardware_id,
            connected: true,
        })
    })
}

#[cfg(windows)]
fn set_default_device(
    device_id: &str,
    expected_direction: wasapi::Direction,
) -> AudioServiceResult<()> {
    use com_policy_config::{IPolicyConfig, PolicyConfigClient};
    use windows::core::PCWSTR;
    use windows::Win32::Media::Audio::{eCommunications, eConsole, eMultimedia};
    use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

    if device_id.trim().is_empty() {
        return Err("The Windows audio endpoint ID cannot be empty".to_string());
    }

    // Validate the endpoint ID and its direction before calling the private
    // policy interface. This prevents accidentally assigning a microphone as
    // the playback default (or vice versa).
    let enumerator = wasapi::DeviceEnumerator::new().map_err(|error| error.to_string())?;
    let device = enumerator
        .get_device(device_id)
        .map_err(|error| format!("Windows audio endpoint '{device_id}' was not found: {error}"))?;

    if device.get_direction() != expected_direction {
        return Err(format!(
            "Endpoint '{device_id}' has direction {}, expected {}",
            device.get_direction(),
            expected_direction
        ));
    }

    let wide_id: Vec<u16> = device_id.encode_utf16().chain(std::iter::once(0)).collect();
    let endpoint_id = PCWSTR(wide_id.as_ptr());

    let policy: IPolicyConfig = unsafe {
        CoCreateInstance(&PolicyConfigClient, None, CLSCTX_ALL)
    }
    .map_err(|error| format!("Could not access Windows audio policy: {error}"))?;

    // Windows keeps separate Console, Multimedia and Communications defaults.
    // SoundSwaper intentionally moves all three roles so applications agree
    // on the selected input/output endpoint.
    for role in [eConsole, eMultimedia, eCommunications] {
        unsafe { policy.SetDefaultEndpoint(endpoint_id, role) }
            .map_err(|error| format!("Could not set the {role:?} default endpoint: {error}"))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_placeholder_is_never_marked_connected() {
        let device = unavailable_device("Unavailable");
        assert!(!device.connected);
        assert!(device.hardware_id.is_empty());
    }

    #[test]
    fn unknown_device_type_returns_no_devices() {
        assert!(get_audio_devices_by_type("unknown").is_empty());
    }

    #[test]
    fn empty_device_id_is_rejected_by_public_setters() {
        assert!(set_default_output_device("").is_err());
        assert!(set_default_input_device("").is_err());
    }
}
