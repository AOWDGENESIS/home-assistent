param(
    [string]$InstallRoot = (Join-Path $env:LOCALAPPDATA "UniversalRecovery")
)

$ErrorActionPreference = "Stop"
$resolvedRoot = [System.IO.Path]::GetFullPath($InstallRoot)
if (Test-Path -LiteralPath $resolvedRoot) {
    Remove-Item -LiteralPath $resolvedRoot -Recurse -Force
    Write-Host "UniversalRecovery entfernt: $resolvedRoot"
} else {
    Write-Host "Keine Installation gefunden: $resolvedRoot"
}
