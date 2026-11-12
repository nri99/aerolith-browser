$Root = $PSScriptRoot
$candidates = @(
    (Join-Path $Root "target\release\aerolith-browser.exe"),
    (Join-Path $Root "target\x86_64-pc-windows-gnu\release\aerolith-browser.exe")
)

$exePath = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $exePath) {
    Write-Error "Release executable not found. Please compile first via 'cargo build --release'"
    exit 1
}

$desktop = [Environment]::GetFolderPath('Desktop')
$lnkPath = Join-Path $desktop 'Aerolith Browser.lnk'
$wsh = New-Object -ComObject WScript.Shell
$lnk = $wsh.CreateShortcut($lnkPath)
$lnk.TargetPath = $exePath
$lnk.WorkingDirectory = $Root
$lnk.Description = 'Aerolith - Ultra-Lightweight Rust Browser (Surface Go 2 Optimized)'
$lnk.Save()
Write-Host "Created Desktop Shortcut: $lnkPath (target: $exePath)" -ForegroundColor Green
