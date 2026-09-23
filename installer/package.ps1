param(
    [string]$Output = (Join-Path (Join-Path $PSScriptRoot "..") "dist\UniversalRecovery-0.1.0-source.zip")
)

$ErrorActionPreference = "Stop"
$repo = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$outputPath = [System.IO.Path]::GetFullPath($Output)
$stage = Join-Path ([System.IO.Path]::GetTempPath()) ("UniversalRecovery-" + [guid]::NewGuid())

try {
    New-Item -ItemType Directory -Force -Path $stage | Out-Null
    $items = @(
        "Cargo.toml", "Cargo.lock", "README.md", "maintainer-check.md",
        "ur-core", "ur-cli", "ur-recovery", "ur-device-broker", "installer"
    )
    foreach ($item in $items) {
        $source = Join-Path $repo $item
        if (Test-Path -LiteralPath $source) {
            Copy-Item -LiteralPath $source -Destination $stage -Recurse -Force
        }
    }
    $destinationDir = Split-Path -Parent $outputPath
    New-Item -ItemType Directory -Force -Path $destinationDir | Out-Null
    if (Test-Path -LiteralPath $outputPath) {
        Remove-Item -LiteralPath $outputPath -Force
    }
    Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $outputPath -CompressionLevel Optimal
    Write-Host "Paket erstellt: $outputPath"
} finally {
    if (Test-Path -LiteralPath $stage) {
        Remove-Item -LiteralPath $stage -Recurse -Force
    }
}
