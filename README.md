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
- The Windows WASAPI backend is **not implemented yet**. The audio service currently returns no devices rather than fabricated hardware.
- Global shortcuts and system-tray integration are not implemented yet.
- The frontend is a scaffold; configuration management and device switching are not wired to Rust commands yet.

The project is under active development. Do not treat the feature list of the original .NET application as a statement that all features are available in this Rust rewrite.
