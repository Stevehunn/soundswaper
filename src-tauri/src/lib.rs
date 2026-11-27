/// SoundSwap's reusable application services.
#[path = "../../src/core/lib.rs"]
pub mod core;

/// Run the SoundSwap desktop application.
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running SoundSwap");
}
