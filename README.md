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
- **Windows audio endpoint discovery is implemented** through the Core Audio MMDevice API: active playback and capture endpoints, friendly names, stable endpoint IDs, and current Console defaults.
- **Changing the Windows default input/output endpoint is implemented** through the COM `IPolicyConfig` interface. This interface is undocumented by Microsoft, so Windows-version compatibility must be verified on physical Windows machines. SoundSwaper currently applies the selected endpoint to the Console, Multimedia, and Communications roles.
- Audio endpoint listing, default-device lookup, and device switching are exposed as Tauri commands and connected to the frontend selectors.
- Global shortcuts and system-tray integration are not implemented yet.

The project is under active development. Do not treat the feature list of the original .NET application as a statement that all features are available in this Rust rewrite.

## Audio backend notes

The endpoint IDs returned by Windows are opaque identifiers. Persist `AudioDevice.hardware_id` and pass it to `set_default_output_device` or `set_default_input_device`; do not identify devices by friendly name because names are not guaranteed to be unique.

The error-aware `try_*` functions should be preferred by application code. The older convenience functions remain for compatibility and return an empty list or a disconnected placeholder when the Windows audio service reports an error.
