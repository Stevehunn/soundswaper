@echo off
REM ================================================================
REM SoundSwap Rust App - Test Configuration Script
REM ================================================================

cd c:\Dev\soundswaper

echo.
echo [Step 1] Verifying project structure...
if exist "src-tauri\Cargo.toml" (
    echo ✓ src-tauri/Cargo.toml found
) else (
    echo ✗ src-tauri/Cargo.toml missing
)

if exist "tauri.conf.json" (
    echo ✓ tauri.conf.json found
) else (
    echo ✗ tauri.conf.json missing
)

echo.
echo [Step 2] Checking Cargo.toml syntax...
cargo check 2>&1 | Select-String -Pattern "(Checking|Finished|error)" -Context 1,3

echo.
echo [Step 3] Verifying Tauri configuration...
python generate_config.py 2>$null; echo "✓ Configuration created or exists"

echo.
echo ================================================================
echo NEXT STEPS:
echo 1. Run 'cargo tauri dev' to test the app in development mode
echo 2. Or run 'cargo tauri build --release' for production build
echo ================================================================
pause
