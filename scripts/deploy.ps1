# Deployment-Skript für Home Assistant Konfigurationen

Write-Host "Starte Deployment..." -ForegroundColor Cyan

# Prüfe, ob der configuration-Ordner existiert
if (-Not (Test-Path "./configuration")) {
    Write-Host "Fehler: Der Ordner 'configuration' wurde nicht gefunden." -ForegroundColor Red
    exit 1
}

# Beispiel: Kopieren der Konfigurationen in ein Zielverzeichnis
$target = "C:\HomeAssistant\config"

if (-Not (Test-Path $target)) {
    Write-Host "Erstelle Zielordner: $target" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $target | Out-Null
}

Write-Host "Kopiere Konfigurationen..." -ForegroundColor Yellow
Copy-Item -Path "./configuration/*" -Destination $target -Recurse -Force

Write-Host "Deployment abgeschlossen." -ForegroundColor Green
