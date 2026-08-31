# ADR 0002: Device Structure

## Status
Accepted

## Kontext
Die Geräte in Home Assistant sollen klar strukturiert, nachvollziehbar und erweiterbar definiert werden.  
Dazu wird eine zentrale Datei `devices.yaml` verwendet.

## Entscheidung
- Geräte werden in einer einzigen Datei (`configuration/devices.yaml`) verwaltet.
- Jedes Gerät erhält:
  - `name`
  - `id`
  - `type`
- Die Struktur bleibt bewusst einfach, um spätere Automationen und Skripte nicht zu verkomplizieren.

## Konsequenzen
- Einheitliche Geräteverwaltung
- Leichte Erweiterbarkeit
- Klare Referenzen für Automationen und Skripte
