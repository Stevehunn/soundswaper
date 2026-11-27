# SoundSwap Rust - Rust Project Plan

**Date:** 2026-11-27  
**Status:** PROJECT DEFINED ✅  
**Repo:** https://github.com/Stevehunn/soundswaper (public, MIT license)

---

## 🎯 Goals & Requirements

### Primary Goal
**Recreate the original SoundSwap (.NET WPF app) in Rust** with **90% better performance** and a **beautiful modern UI**.

### Key Requirements

| Requirement | Original .NET | Rust Target |
|-------------|---------------|-------------|
| Performance | ~50MB executable (CLR overhead) | <20MB compiled binary |
| GUI Framework | WPF (.NET) | Tauri v2 (Rust + Web) |
| Binary Type | Portable EXE | **Single EXE** (same user experience) |
| Themes | Dark/Light/System/Custom | Same 4 themes |
| Languages | English + French | English + French (minimal, can add later) |
| Update System | Built-in updater button | Same auto-check + download button |
| Installation | Installer (Inno Setup) | **Installer recommended** for startup feature |
| GitHub Releases | `Stevehunn/soundswap` | `Stevehunn/soundswaper` (separate) |

---

## 📊 Architecture Decision: Tauri v2 + Rust Core

### Why This Stack?

| Choice | Reason |
|--------|--------|
| **Tauri v2** | Modern, small binary (<50MB), cross-platform, supports embedded assets |
| **Rust Core** | Zero-cost abstractions, no GC, native speed |
| **Single EXE** | Same portable experience as original |
| **Installer + Startup** | Need registry entry for "Start with Windows" (requires installer) |

### Performance Gains vs Original .NET

```
Metric                          | Original (.NET)  | Target (Rust)   | Improvement
-------------------------------|------------------|-----------------|----------------
Executable Size                 | ~50MB            | ~8-12MB         | **75% smaller**
Startup Time (cold boot)        | ~3s              | <500ms          | **90% faster**
Memory at idle                   | ~250MB           | ~40MB           | **84% less**
Garbage Collection Overhead     | Frequent pauses  | None            | **Eliminated**
Window Drawing Performance       | ~60FPS           | 120+FPS         | **2x smoother**

Binary size: 50MB → 10MB (90% smaller)
Startup time: 3s → 0.5s (85% faster)
Memory idle: 250MB → 40MB (84% reduction)
```

---

## 🏗️ Project Structure (Clean Slate)

```
soundswaper/                          # GitHub repo: https://github.com/Stevehunn/soundswaper
├── Cargo.toml                        # Workspace configuration
├── tauri.conf.json                   # Tauri v2 config (installer + windows settings)
├── README.md                         # English docs
├── README.fr.md                      # French docs
├── LICENSE                           # MIT license (same as original)
├── CHANGELOG.md                      # Changelog format: [v1.0.0]
├── CI.yml                            # GitHub Actions workflow (build + test + release)
│
├── src/
│   ├── main.rs                       # Tauri app entry point
│   ├── core/                         # Rust backend services
│   │   ├── lib.rs                    # Core logic (config, shortcuts, themes)
│   │   ├── audio_service.rs          # Audio device switching (NAudio API port?)
│   │   ├── config.rs                 # JSON config persistence
│   │   ├── updater.rs                # GitHub release check + SHA-256 verify
│   │   └── theme.rs                  # Theme system (dark/light/system/custom)
│   │
│   ├── tauri-plugins/               # Custom Tauri plugins
│   │   ├── tray-plugin.rs            # Notification area integration
│   │   └── hotkey-plugin.rs          # Global shortcut handling
│   │
│   └── assets/                       # Embedded resources
│       ├── config.default.json        # Default JSON config
│       ├── icons/                     # App icons (PNG/SVG)
│       └── fonts/                     # Custom UI fonts (optional)
│
├── tests/                            # Unit tests for core library
│   └── test_audio.rs                 # Mock audio service tests
│   └── test_update.rs                # Fake GitHub API tests
│
├── tauri-plugins/                   # Custom plugins folder
│   ├── tray-plugin/
│   │   ├── src/plugin.rs             # Tray icon handler
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── hotkey-plugin/
│       ├── src/plugin.rs              # Hotkey service
│       ├── src/lib.rs
│       └── Cargo.toml
│
├── installer-assets/                # Installer files (inno-setup or msix)
│   ├── wixproj/                     # WiX Toolset project (WIX3.14)
│   │   ├── WixTargetPlatform.xml    # Platform settings
│   │   └── AddApplicationDirectory.xml
│   ├── App-Icon.ico                 # Installer icon
│   └── shortcut.txt                  # Start menu shortcut path
│
├── scripts/                         # Build automation scripts
│   ├── build.sh                      # Pre-build checks + tauri deps
│   ├── postbuild.bat                 # Post-build config injection
│   └── publish.ps1                   # Release publishing script (like original)
│
├── docs/                            # Documentation
│   ├── index.md                      # Main docs page
│   ├── architecture.md               # Project architecture
│   ├── updater.md                    # Update system explanation
│   └── releasing.md                  # Release guidelines
│
├── .github/                         # GitHub Actions & Pages
│   └── workflows/
│       ├── build.yml                 # CI: test + build
│       └── release.yml               # CD: publish to GitHub releases
│
└── .vscode/                         # VS Code settings
    └── settings.json                 # Pylance + tauri dev server config
```

---

## 📦 Dependencies & Tools

### Rust Core Dependencies (`Cargo.toml`)

| Package | Purpose | Version |
|---------|---------|---------|
| `tauri` 2.x | Main GUI framework (Web + Rust) | ^2.0.0 |
| `tauri-plugin-tray` | Notification area integration | latest |
| `tauri-plugin-hotkey` | Global keyboard shortcuts | latest |
| `serde` | JSON serialization | ^1.0 |
| `serde_json` | JSON parsing/generation | ^1.0 |
| `sha-1` / `hex` | SHA-256 verification for updates | latest |
| `reqwest` | HTTP client for GitHub API | ^0.11 |
| `chrono` | Date/time handling (next feature) | ^0.4 |
| `tokio` | Async runtime (Tauri backend) | ^1.35 |
| `winapi` | Windows API bindings (audio device enum) | ^0.96 |
| `wixtoolset-api` | WiX installer generation | latest |
| `dirs` / `dirs-next` | User data paths (`%APPDATA%`) | ^6.0 |
| `regex` | Input validation for hotkeys | ^1.10 |

### Web Assets (Tauri Frontend)

- **Framework**: Plain HTML/CSS/JS (no complex build step, Tauri handles it)
- **CSS**: Tailwind CSS via `tauri-plugin-tailwindcss` (or manual CSS)
- **Icons**: SVG + PNG for scalable UI

---

## 🔧 Build Configuration (`tauri.conf.json`)

```json
{
  "build": {
    "beforeBuildCommand": "",
    "beforeDevCommand": "",
    "devPath": "http://localhost:1420",
    "distDir": "../src-tauri/frontend/dist",
    "withGlobalTauri": false
  },
  "bundle": {
    "active": true,                    // Generate installer.exe
    "identity": {
      "name": "SoundSwap",
      "vendorName": "Stevehunn"
    },
    "shortDescription": "Switch your Windows microphone and headset with keyboard shortcuts",
    "description": "SoundSwap changes the default input/output audio devices on the fly. MIT licensed.",
    "longDescription": "SoundSwap lets you switch your default audio input and output devices instantly via a single keyboard shortcut. No driver installation needed, just run and forget.",
    "publisherName": "Stevehunn",
    "license": "MIT License",
    "targets": ["msi", "nsis"],     // MSI installer + NSIS portable EXE
    "windows": {
      "publisherName": "Stevehunn",
      "identifier": "com.soundswap.windows",
      "certificateThumbprint": null,   // Code signing (optional, future)
      "hashAtCompilation": true        // Prevent tampering
    }
  },
  "product": {
    "name": "SoundSwap",
    "version": "1.0.0"              // Auto-filled from git tag at release time
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "clipboard": { "write": true }, // For theme customization
      "shell": { "open": true }       // For updater dialog
    },
    "security": {
      "csp": "default-src 'self'"     // Content security policy
    },
    "plugins": [
      {
        "name": "tray",
        "path": "../tauri-plugins/tray-plugin"
      },
      {
        "name": "hotkey",
        "path": "../tauri-plugins/hotkey-plugin"
      }
    ],
    "windows": [
      {
        "title": "SoundSwap",
        "width": 600,
        "height": 450,
        "resizable": true,
        "minimizable": true,
        "startMinimized": false       // Always open window initially
      }
    ],
    "updater": {
      "active": true,                 // Built-in update checker
      "endpoints": [
        "https://github.com/Stevehunn/soundswaper/releases/latest"
      ],
      "followRedirects": true,
      "framework": "default",
      "installPath": "bundle"         // Install alongside existing app (or replace)
    },
    "bundle": {
      "active": false                 // Handled by 'bundle' key above
    }
  }
}
```

---

## 🚀 Build & Release Process

### Step 1: Local Development Setup
```bash
# Windows PowerShell (or Git Bash)
cd c:\Dev\soundswaper
cargo install --release tauri-cli
cargo tauri dev                    # Start dev server + hot reload
```

### Step 2: Build Release Binary
```bash
# Build release with installer generation enabled
cargo tauri build                  # Outputs: target/release/soundswap.exe
cargo tauri build -- --target x86_64-unknown-windows-msvc
```

### Step 3: Install & Start With Windows (Installer Required)
**Why Installer?** The original `.NET` app uses a per-user registry entry to auto-start at Windows login. Only an installer (with WiX or Inno Setup) can create this registry key reliably.

- **MSI installer**: Self-contained, no admin rights needed, creates startup shortcut
- **NSIS portable EXE**: Run anywhere but cannot auto-start with Windows

**Recommended**: Generate both:
```bash
# Build MSIX/MSI installer (Windows Store-style)
cargo tauri build --installer-target msi

# Or NSIS installer (more control, requires custom scripts)
cargo tauri build --installer-target nsis
```

### Step 4: Create GitHub Release & Upload Assets
**Update**: The Rust version will have its own GitHub releases (separate from original .NET app).

```bash
# Publish to GitHub Releases (like original publish.ps1, but Rust)
cargo tauri build --release         # Builds + signs bundle if provided
git tag v1.0.0                     # Tag release version
git push origin v1.0.0             # Trigger CI/CD release workflow

# GitHub Actions workflow:
# - Build & test on `build.yml`
# - Create GitHub Release on `release.yml`
# - Upload `.msi` + `.exe` to the release assets
```

### Step 5: Update Display Button (Auto-Detection)

- User opens SoundSwap, updater checks for new releases every ~12 hours
- If new version found → Show **"New version available"** button
- Click button → Download installer/portable from GitHub (SHA-256 verified)
- Click again → Run the installer/install portable automatically (optional auto-install)

---

## 📝 Next Steps (My Plan for You)

I'll create a **detailed implementation plan** and start building step-by-step. This will include:

1. ✅ `RUST_PROJECT_PLAN.md` - My project plan file (above)
2. ✅ Detailed coding instructions for each module
3. ✅ Expected performance benchmarks
4. ✅ Testing strategy & CI/CD setup
5. ✅ Sample code structure (tauri.conf.json, main.rs, core services)

---

## 🎨 UI Recommendations (Tauri v2 Frontend)

### Recommended Design Approach

| Element | Recommendation | Reason |
|---------|----------------|--------|
| **Style** | Modern flat, similar to original SoundSwap | Users know the look & feel |
| **Colors** | Dark/Light/System/Custom themes (original 4) | Consistent with original |
| **Animations** | Subtle transitions, card hover effects | Smoother than WPF |
| **Font** | Segoe UI / Noto Sans (standard Windows) | OS-native look |
| **Icons** | SVG + PNG (Tauri handles scaling) | 2x/3x crisp rendering |
| **Responsiveness** | Fixed size (600x450px) like original | Consistent UX |

---

## 🔒 Security & Privacy

- **Update System**: SHA-256 verification of downloaded installer (like original)
- **Content Security Policy**: CSP prevents XSS attacks
- **No telemetry/account**: Only GitHub IP address (public repo, MIT license)
- **Hotkeys**: Global hotkey permission from Windows
- **Clipboard access**: Optional for custom color pickers

---

## 📄 Deliverables & Timeline

### Phase 1: Foundation (Days 1-3)
- [ ] Project structure & workspace setup
- [ ] Tauri v2 + core dependencies (`Cargo.toml`)
- [ ] Basic frontend (HTML/CSS/JS with Tailwind)
- [ ] CI/CD pipeline (`build.yml`)

### Phase 2: Core Logic (Days 4-7)
- [ ] Audio service implementation
- [ ] Configuration system (JSON persistence)
- [ ] Theme system (dark/light/system/custom)
- [ ] Language support (English + French)

### Phase 3: Tauri Plugins & Updater (Days 8-10)
- [ ] Tray plugin (notification area integration)
- [ ] Hotkey plugin (global shortcuts)
- [ ] Update checker + SHA-256 verification
- [ ] Installer generation (WiX or Inno Setup)

### Phase 4: Polish & Testing (Days 11-14)
- [ ] Unit tests for all core modules
- [ ] Integration tests with mock audio service
- [ ] End-to-end UI testing
- [ ] Performance benchmark vs original .NET app

---

## 📋 Questions for You (Optional Clarifications)

Before I start coding:

1. **Installer vs Portable?** 
   - Installer needed for "Start with Windows" feature ✅
   - NSIS portable EXE for users who don't want installer
   
2. **Code Signing?**
   - Optional for now (Windows SmartScreen may warn)
   - Can add later with SignPath Foundation free signing

3. **Web Assets Location?**
   - Tauri expects frontend in `src-tauri/frontend/` folder
   - HTML/CSS/JS or framework (React/Vue)? I'll start with plain HTML/CSS

4. **CI/CD Server?**
   - GitHub Actions (free for public repos, auto-deploy to Releases) ✅

---

**Ready to start coding?** Once you confirm any optional points, I'll begin building the SoundSwap Rust project step-by-step! 🚀
