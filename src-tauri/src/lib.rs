/// SoundSwap's reusable application services.
#[path = "../../src/core/lib.rs"]
pub mod core;

use core::audio_service::{self, AudioDevice};

/// Return active endpoints for the requested direction ("input" or "output").
#[tauri::command]
fn get_audio_devices(device_type: String) -> Result<Vec<AudioDevice>, String> {
    match device_type.as_str() {
        "input" | "output" => audio_service::try_get_audio_devices_by_type(&device_type),
        _ => Err("Audio device type must be 'input' or 'output'".to_string()),
    }
}

/// Return the current default endpoint for the requested direction.
#[tauri::command]
fn get_default_audio_device(device_type: String) -> Result<AudioDevice, String> {
    match device_type.as_str() {
        "input" => audio_service::try_get_default_input_device(),
        "output" => audio_service::try_get_default_output_device(),
        _ => Err("Audio device type must be 'input' or 'output'".to_string()),
    }
}

/// Change the default endpoint for the requested direction.
#[tauri::command]
fn set_default_audio_device(device_type: String, device_id: String) -> Result<(), String> {
    match device_type.as_str() {
        "input" => audio_service::set_default_input_device(&device_id),
        "output" => audio_service::set_default_output_device(&device_id),
        _ => Err("Audio device type must be 'input' or 'output'".to_string()),
    }
}

/// Run the SoundSwap desktop application.
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_audio_devices,
            get_default_audio_device,
            set_default_audio_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running SoundSwap");
}
