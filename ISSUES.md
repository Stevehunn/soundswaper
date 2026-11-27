# SoundSwap Rust Project - Configuration & Build Issues

**Date:** 2026-11-27  
**Workspace:** `c:\Dev\soundswaper`

---

## 🔍 PROBLEM SUMMARY

The project has **corrupted Tauri v2 configuration** that prevents building. Multiple attempts to fix the JSON structure have failed, causing file corruption and build failures.

### Errors Encountered:
```
1. "Additional properties are not allowed ('windows', 'splashScreen', 'updater' were unexpected)"
2. "unexpected argument '--release' found" 
3. "no targets specified in the manifest - either src/lib.rs, src/main.rs must be present"
4. Tauri library import errors when building
```

### Root Causes:
1. **Tauri v2 Configuration Structure** - Properties (`windows`, `splashScreen`, `updater`) need to be nested inside the `tauri` object, not at root level
2. **Cargo.toml Workspace Structure** - Incorrect workspace setup with missing proper library targets
3. **Build Command Syntax** - `cargo tauri build --release` is incorrect; should use `cargo tauri build` (Tauri 2.12 uses different syntax)
4. **Missing Tauri Dependencies** - Tauri crate not properly declared in `src-tauri/Cargo.toml`

---

## 📋 REQUIRED FILES & STRUCTURE

### Correct Project Layout:
```
c:\Dev\soundswaper\
├── Cargo.toml                          # Root cargo file (optional)
├── src-tauri\                          # Tauri v2 source directory
│   ├── Cargo.toml                      # Tauri app dependencies
│   └── src\lib.rs                      # Rust library code
├── src-tauri\frontend\index.html      # Web frontend (EXISTS)
├── tauri.conf.json                     # Configuration file (CORRUPTED)
└── assets\                              # Application assets (EXISTS)
```

### Required Cargo.toml Structure (`src-tauri/Cargo.toml`):
```toml
[package]
name = "soundswap-tauri"
version = "0.1.0"
edition = "2021"

# Tauri v2 requires this build script dependency
[build-dependencies]
tauri-build = { version = "2.0", features = [] }

# Correct dependencies for Tauri v2 (NOT workspace-based)
[dependencies]
tauri = { version = "2.0", features = ["all"] }
serde = { version = "1.0", features = ["derive"] }
sha1.workspace = true  # or just use "sha1.workspace = true"
hex.workspace = true
reqwest.workspace = true
chrono.workspace = true
tokio.workspace = true

# Windows API
winapi = { version = "0.96" }

# Configuration management
dirs = { version = "6.0" }
```

### Correct tauri.conf.json Structure:
```json
{
  "productName": "SoundSwaper",
  "version": "1.0.0",
  "identifier": "com.soundswaper.windows",
  "build": {
    "beforeBuildCommand": "",
    "beforeDevCommand": ""
  },
  "bundle": {
    "active": true,
    "targets": ["msi"]
  },
  "tauri": {
    // Properties go INSIDE the tauri object for Tauri v2
    // Note: splashScreen/windows/updater should be empty initially
  }
}
```

---

## 🛠️ BUILD COMMANDS TO USE

### Install Tauri CLI (Required First):
```bash
cargo install cargo-tauri-cli
```

### Test Development Mode (Hot-reloading):
```bash
cd c:\Dev\soundswaper && cargo tauri dev
```

### Test Production Build:
```bash
cd c:\Dev\soundswaper && cargo tauri build --release
```

**Important:** Do NOT use `--release` as a flag. Use `cargo tauri build --release` (with double dash) OR just `cargo tauri build` in Tauri 2.x - it defaults to release mode.

---

## 📁 CURRENT STATUS OF FILES

| File | Status | Notes |
|------|--------|-------|
| `Cargo.toml` | ✅ Correct | Root Cargo file is fine |
| `src-tauri/Cargo.toml` | ⚠️ NEEDS FIX | Uses workspace dependencies incorrectly |
| `src-tauri/src/lib.rs` | ❌ NEEDS IMPLEMENTATION | Currently has placeholder code with tauri imports |
| `frontend/index.html` | ✅ EXISTS | Already created (but may need cleanup) |
| `tauri.conf.json` | ❌ CORRUPTED | Multiple test iterations created invalid structure |
| `assets/banners/Sounswap-logo.jpg` | ⚠️ NEEDS VERIFICATION | Must be .jpg format, not .png |

---

## 🔑 KEY ISSUES TO EXPLAIN TO ASSISTANT

1. **Tauri v2 Schema** - Need to explain that `windows`, `splashScreen`, and `updater` must be inside the `tauri` object:
   ```json
   {
     "tauri": {
       "windows": [{"title": "SoundSwaper"}],
       "splashScreen": {"imageSrc": "/assets/banners/Sounswap-logo.jpg", "time": 5000}
     }
   }
   ```

2. **Workspace Dependencies** - Tauri 2.12 can't use `[dependencies]` from root `Cargo.toml`. Each crate needs explicit dependencies declared in its own `Cargo.toml`.

3. **Build Script Dependency** - Must have `tauri-build = { version = "2.0", features = [] }` in the app's `Cargo.toml`.

4. **File Naming Convention** - Asset files use `.jpg` extension, not `.png`, so frontend must reference:
   ```html
   <img src="/assets/banners/Sounswap-logo.jpg" alt="SoundSwap Logo">
   ```

---

## 🚀 NEXT STEPS FOR FIXING

1. ✅ **Install Tauri CLI**: `cargo install cargo-tauri-cli`
2. ✅ **Fix `src-tauri/Cargo.toml`**: Replace workspace-based dependencies with direct ones
3. ✅ **Create proper tauri.conf.json** (empty `tauri` object initially)
4. ✅ **Implement Rust code** in `src-tauri/src/lib.rs` (not just placeholders)
5. ✅ **Verify asset files exist** at correct paths
6. ✅ **Run `cargo tauri dev`** to test

---

## 📝 ADDITIONAL NOTES

- The project is for SoundSwaper (GitHub repo: Stevehunn/soundswaper)
- Original .NET app name was "SoundSwap" (without 'er')
- Product must use "SoundSwaper" in configuration
- Asset files are in `assets/banners/` folder (Sounswap-logo.jpg, sounswap-icon.ico)
- Frontend HTML file is in `src-tauri/frontend/index.html`

---

## 🔗 RELEVANT RESOURCES

- **Tauri v2 Documentation**: https://v2.tauri.app/
- **Tauri CLI Help**: `cargo tauri --help`
- **Cargo Tauri GitHub**: https://github.com/tauri-apps/cargo-tauri-cli

---

**Last Updated:** 2026-11-27  
**Status:** Awaiting external help to complete fix
