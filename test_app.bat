@echo off
echo.
echo ================================================================
echo Testing SoundSwap Rust App (Tauri v2)
echo ================================================================
echo.

cd c:\Dev\soundswaper

echo [Step 1] Verifying project structure...
if exist "src-tauri\src\lib.rs" (
    echo ✓ lib.rs found
) else (
    echo ✗ lib.rs missing - creating placeholder...
    mkdir src-tauri\src 2>$null; Write-Output "# Tauri v2 Library - Placeholder content needed" > src-tauri\src\lib.rs
)

echo.
echo [Step 2] Running cargo check (syntax validation)...
cargo check --workspace 2>&1 | Select-String -Pattern "(Checking|Error)" -Context 2,3

echo.
echo [Step 3] Verifying tauri.conf.json structure...
python -c "import json; d=json.load(open('tauri.conf.json')); print(f'✓ Product: {d[\"productName\"]}')"

echo.
echo ================================================================
echo Testing complete! Run 'cargo tauri dev' to test the app
echo ================================================================
pause
