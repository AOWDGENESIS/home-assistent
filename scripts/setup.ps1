# Setup-Skript für Home Assistant Konfigurationen

Write-Host "Starte Setup..." -ForegroundColor Cyan

# Beispiel: Pfadprüfung
if (-Not (Test-Path "./configuration")) {
    Write-Host "Fehler: Der Ordner 'configuration' wurde nicht gefunden." -ForegroundColor Red
    exit 1
}

Write-Host "Setup abgeschlossen." -ForegroundColor Green
