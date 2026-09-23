param(
    [string]$InstallRoot = (Join-Path $env:LOCALAPPDATA "UniversalRecovery")
)

$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")

$binary = Join-Path (Get-Location) "target\release\ur-cli.exe"
if (-not (Test-Path -LiteralPath $binary)) {
    throw "Keine geprüfte Release-Binary gefunden. Zuerst .\installer\build.ps1 ausführen."
}

$resolvedRoot = [System.IO.Path]::GetFullPath($InstallRoot)
New-Item -ItemType Directory -Force -Path $resolvedRoot | Out-Null
Copy-Item -LiteralPath $binary -Destination (Join-Path $resolvedRoot "ur-cli.exe") -Force
Copy-Item -LiteralPath (Join-Path $PSScriptRoot "README_DE.txt") `
    -Destination (Join-Path $resolvedRoot "README_DE.txt") -Force

$installed = Join-Path $resolvedRoot "ur-cli.exe"
& $installed selftest
if ($LASTEXITCODE -ne 0) {
    throw "Installation wurde nicht verifiziert; Selbsttest fehlgeschlagen."
}

Write-Host "UniversalRecovery installiert nach: $resolvedRoot"
