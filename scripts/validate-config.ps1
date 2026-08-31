# Validierung der Home Assistant YAML-Konfigurationen

Write-Host "Starte YAML-Validierung..." -ForegroundColor Cyan

# Prüfe, ob der configuration-Ordner existiert
if (-Not (Test-Path "./configuration")) {
    Write-Host "Fehler: Der Ordner 'configuration' wurde nicht gefunden." -ForegroundColor Red
    exit 1
}

# Prüfe alle YAML-Dateien im configuration-Ordner
Get-ChildItem -Path "./configuration" -Filter *.yaml | ForEach-Object {
    Write-Host "Prüfe Datei: $($_.Name)" -ForegroundColor Yellow

    try {
        $content = Get-Content $_.FullName -Raw
        $null = $content | ConvertFrom-Yaml
        Write-Host "✓ YAML gültig: $($_.Name)" -ForegroundColor Green
    }
    catch {
        Write-Host "✗ Fehler in Datei $($_.Name): $_" -ForegroundColor Red
    }
}

Write-Host "Validierung abgeschlossen." -ForegroundColor Green
