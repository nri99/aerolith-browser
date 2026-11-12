@echo off
set "BIN=%~dp0target\release\aerolith-browser.exe"
if not exist "%BIN%" set "BIN=%~dp0target\x86_64-pc-windows-gnu\release\aerolith-browser.exe"
if exist "%BIN%" (
    start "" "%BIN%"
) else (
    echo [ERROR] Aerolith Browser binary not found.
    echo Please run: cargo build --release
    pause
)
