$ErrorActionPreference = "Stop"

Set-Location (Join-Path $PSScriptRoot "..")
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "Rust/Cargo fehlt. Installiere rustup von https://rustup.rs/."
}

$linker = Get-Command link.exe -ErrorAction SilentlyContinue
if (-not $linker) {
    throw "MSVC link.exe fehlt. Installiere Visual Studio 2022 mit 'Desktopentwicklung mit C++'."
}

cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo build --release -p ur-cli

$binary = Join-Path (Get-Location) "target\release\ur-cli.exe"
if (-not (Test-Path -LiteralPath $binary)) {
    throw "Release-Binary wurde nicht erzeugt: $binary"
}

& $binary selftest
if ($LASTEXITCODE -ne 0) {
    throw "CLI-Selbsttest fehlgeschlagen."
}

Write-Host "Build und Selbsttest erfolgreich: $binary"
