/// SoundSwap's reusable application services.
#[path = "../../src/core/lib.rs"]
pub mod core;

use crate::core::audio_service::{self, AudioDevice};
use crate::core::config::{self, Configuration};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_autostart::ManagerExt as AutostartManagerExt;
use tauri_plugin_global_shortcut::{
    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
};

/// Load the user's persisted settings and audio profiles.
#[tauri::command]
fn load_configuration(app: tauri::AppHandle) -> Result<Configuration, String> {
    let directory = app
        .path()
        .app_config_dir()
        .map_err(|error| format!("Could not resolve the application config directory: {error}"))?;
    let directory = directory.to_string_lossy().to_string();
    std::fs::create_dir_all(&directory)
        .map_err(|error| format!("Could not create the application config directory: {error}"))?;
    Ok(config::load_config(&directory))
}

/// Persist settings and profiles in the user's application config directory.
#[tauri::command]
fn save_configuration(
    app: tauri::AppHandle,
    mut configuration: Configuration,
) -> Result<(), String> {
    let directory = app
        .path()
        .app_config_dir()
        .map_err(|error| format!("Could not resolve the application config directory: {error}"))?;
    let directory = directory.to_string_lossy().to_string();
    std::fs::create_dir_all(&directory)
        .map_err(|error| format!("Could not create the application config directory: {error}"))?;
    configuration.data_path = directory.clone();
    config::save_config_to_path(&configuration, &directory)
        .map_err(|error| format!("Could not save SoundSwaper settings: {error}"))
}

#[derive(serde::Serialize)]
struct UpdateCheckResult {
    current_version: String,
    latest_version: String,
    update_available: bool,
    release_url: String,
}

/// Check the latest published GitHub release against the running application version.
#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle) -> Result<UpdateCheckResult, String> {
    let options = crate::core::updater::UpdateOptions::default();
    let current_version = app.package_info().version.to_string();
    let client = reqwest::Client::builder()
        .user_agent("SoundSwaper")
        .build()
        .map_err(|error| format!("Could not create update checker: {error}"))?;

    let release = match crate::core::updater::check_for_updates(&options, client).await {
        Ok(release) => release,
        Err(error) if error.status() == Some(reqwest::StatusCode::NOT_FOUND) => {
            return Err("No published GitHub release is available yet.".to_string());
        }
        Err(error) => return Err(format!("Could not check for updates: {error}")),
    };

    let latest_version = if release.tag_name.is_empty() {
        release.version
    } else {
        release.tag_name
    };
    let update_available =
        crate::core::updater::is_outdated(&current_version, &latest_version);
    let release_url = if release.url.is_empty() {
        options.releases_page_url()
    } else {
        release.url
    };

    Ok(UpdateCheckResult {
        current_version,
        latest_version,
        update_available,
        release_url,
    })
}

/// Return whether SoundSwaper is registered to start when the user signs in to Windows.
#[tauri::command]
fn get_start_with_windows(app: tauri::AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled()
        .map_err(|error| format!("Could not read Windows startup registration: {error}"))
}

/// Enable or disable Windows startup and persist the resulting preference.
#[tauri::command]
fn set_start_with_windows(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    let previous = app.autolaunch().is_enabled()
        .map_err(|error| format!("Could not read Windows startup registration: {error}"))?;
    if previous != enabled {
        let manager = app.autolaunch();
        let result = if enabled { manager.enable() } else { manager.disable() };
        result.map_err(|error| format!("Could not update Windows startup registration: {error}"))?;
    }
    let directory = app.path().app_config_dir()
        .map_err(|error| format!("Could not resolve the application config directory: {error}"))?;
    let directory = directory.to_string_lossy().to_string();
    let mut configuration = config::load_config(&directory);
    configuration.start_with_windows = enabled;
    configuration.data_path = directory.clone();
    if let Err(error) = config::save_config_to_path(&configuration, &directory) {
        let manager = app.autolaunch();
        let rollback = if previous { manager.enable() } else { manager.disable() };
        let rollback_message = rollback.err()
            .map(|e| format!(" Startup rollback also failed: {e}"))
            .unwrap_or_default();
        return Err(format!("Could not save Windows startup preference: {error}.{rollback_message}"));
    }
    Ok(())
}

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

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// Cycle to the next active endpoint and notify the frontend to refresh its labels.
fn cycle_audio_device(app: &tauri::AppHandle, device_type: &str) -> Result<(), String> {
    let devices = audio_service::try_get_audio_devices_by_type(device_type)?;
    if devices.is_empty() {
        return Err(format!("No active {device_type} audio device was found"));
    }

    let current = match device_type {
        "input" => audio_service::try_get_default_input_device()?,
        "output" => audio_service::try_get_default_output_device()?,
        _ => return Err("Audio device type must be 'input' or 'output'".to_string()),
    };

    let current_index = devices
        .iter()
        .position(|device| device.hardware_id == current.hardware_id)
        .unwrap_or(usize::MAX);
    let next_index = if current_index == usize::MAX {
        0
    } else {
        (current_index + 1) % devices.len()
    };

    let next = &devices[next_index];
    match device_type {
        "input" => audio_service::set_default_input_device(&next.hardware_id)?,
        "output" => audio_service::set_default_output_device(&next.hardware_id)?,
        _ => unreachable!(),
    }

    app.emit("soundswap://audio-devices-changed", device_type)
        .map_err(|error| format!("Device changed but UI notification failed: {error}"))?;
    Ok(())
}

fn install_tray(app: &tauri::App) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Open SoundSwaper", true, None::<&str>)?;
    let refresh = MenuItem::with_id(app, "refresh", "Refresh audio devices", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &refresh, &separator, &quit])?;

    let icon_bytes = include_bytes!("../../assets/banners/sounswap-logo.png");
    let icon = tauri::image::Image::from_bytes(icon_bytes)?;

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => show_main_window(app),
            "refresh" => {
                let _ = app.emit("soundswap://audio-devices-changed", "all");
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

/// Run the SoundSwap desktop application.
pub fn run() {
    let show_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyS);
    let next_output_shortcut =
        Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyO);
    let next_input_shortcut =
        Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyI);

    let show_for_handler = show_shortcut.clone();
    let output_for_handler = next_output_shortcut.clone();
    let input_for_handler = next_input_shortcut.clone();

    let app = tauri::Builder::default()
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .app_name("SoundSwaper")
                .build(),
        )
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }

                    if shortcut == &show_for_handler {
                        show_main_window(app);
                    } else if shortcut == &output_for_handler {
                        if let Err(error) = cycle_audio_device(app, "output") {
                            let _ = app.emit("soundswap://shortcut-error", error);
                        }
                    } else if shortcut == &input_for_handler {
                        if let Err(error) = cycle_audio_device(app, "input") {
                            let _ = app.emit("soundswap://shortcut-error", error);
                        }
                    }
                })
                .build(),
        )
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Keep the process and tray icon alive when the user closes the window.
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            load_configuration,
            save_configuration,
            check_for_updates,
            get_start_with_windows,
            set_start_with_windows,
            get_audio_devices,
            get_default_audio_device,
            set_default_audio_device
        ])
        .setup(move |app| {
            install_tray(app)?;

            let shortcuts = [
                (show_shortcut, "show SoundSwaper"),
                (next_output_shortcut, "cycle output audio device"),
                (next_input_shortcut, "cycle input audio device"),
            ];

            for (shortcut, description) in shortcuts {
                app.global_shortcut().register(shortcut).map_err(|error| {
                    Box::<dyn std::error::Error>::from(std::io::Error::other(format!(
                        "Could not register global shortcut to {description}: {error}"
                    )))
                })?;
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building SoundSwaper");

    app.run(|_app_handle, _event| {});
}
