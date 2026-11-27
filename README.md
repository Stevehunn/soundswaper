# SoundSwaper

SoundSwaper is a Windows desktop application being rebuilt in Rust with Tauri 2.

## Development requirements

- Windows 10 or Windows 11
- Stable Rust toolchain (rustup)
- Microsoft C++ Build Tools
- WebView2 Runtime
- Tauri CLI 2

Install the Tauri CLI:

```powershell
cargo install tauri-cli --version "^2" --locked
```

Run the application from the repository root:

```powershell
cargo tauri dev
```

Build the release application:

```powershell
cargo tauri build
```

Run the Rust tests:

```powershell
cargo test
```

## Current implementation status

- Tauri 2 application entry point and frontend loading are configured.
- Configuration settings can be loaded from and saved to a JSON file.
- GitHub release metadata can be queried asynchronously.
- SHA-256 verification is implemented for downloaded file data.
- **Windows audio endpoint discovery** enumerates active playback and capture endpoints, friendly names, stable endpoint IDs, and current Console defaults through Core Audio/MMDevice.
- **Windows default-device switching** uses the undocumented COM `IPolicyConfig` interface. SoundSwaper currently applies the selected endpoint to Console, Multimedia, and Communications roles; test compatibility on supported Windows versions.
- **Tauri audio commands and frontend selectors** expose endpoint listing, default-device lookup, and switching for input and output.
- **System tray** provides Open, Refresh audio devices, and Quit actions. A left click opens the main window.
- **Persistent audio profiles** let users save, apply, and delete input/output device combinations. Profiles are stored as JSON in the user's application configuration directory and retain Windows endpoint IDs.
- **Global shortcuts** are registered while the app is running:
  - `Ctrl + Alt + S`: show the main window.
  - `Ctrl + Alt + O`: switch to the next active output device.
  - `Ctrl + Alt + I`: switch to the next active input device.

The project is under active development. Do not treat the feature list of the original .NET application as a statement that all features are available in this Rust rewrite.

## Audio backend notes

The endpoint IDs returned by Windows are opaque identifiers. Persist `AudioDevice.hardware_id` and pass it to `set_default_output_device` or `set_default_input_device`; do not identify devices by friendly name because names are not guaranteed to be unique.

The error-aware `try_*` functions should be preferred by application code. The older convenience functions remain for compatibility and return an empty list or a disconnected placeholder when the Windows audio service reports an error.

## Windows validation

The audio policy interface and the tray/shortcut behavior still require end-to-end testing on physical Windows 10/11 machines. CI compilation and unit tests cannot verify interactions with the installed audio drivers, other applications' shortcut registrations, or the Windows notification area.
