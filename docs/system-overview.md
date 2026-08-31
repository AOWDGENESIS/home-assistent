# System Overview

Dieses Dokument beschreibt die Gesamtübersicht des Systems, das durch die Home Assistant Konfigurationen in diesem Repository aufgebaut wird.

## Hauptbereiche

### 1. Configuration
Enthält alle YAML-Dateien:
- `base.yaml`
- `devices.yaml`
- `automations.yaml`
- `secrets.example.yaml`

### 2. Scripts
PowerShell-Skripte zur Automatisierung:
- `setup.ps1`
- `validate-config.ps1`
- `deploy.ps1`

### 3. Documentation
Architektur, Komponenten und technische Entscheidungen:
- `architecture.md`
- `components.md`
- ADRs im Unterordner `adr/`

## Ziele des Systems

- Klare Struktur
- Automatisierte Abläufe
- Saubere Trennung von Konfiguration, Skripten und Dokumentation
- Erweiterbarkeit für zukünftige Geräte und Automationen
