# UniversalRecovery

Offline-Datenrettung für Windows 10/11 mit einem Rust-Kern und einer
späteren Desktop-GUI.

## Ziel

UniversalRecovery soll lokale Datenträger erkennen, Dateisysteme analysieren
und gelöschte oder beschädigte Dateien wiederherstellen. Die Verarbeitung
bleibt vollständig lokal: Es gibt keine Cloud, keine Telemetrie und keine
Netzwerkabhängigkeit im Recovery-Kern.

Die Architektur orientiert sich an bewährten Mustern des Schwesterprojekts
`AOWDGENESIS/documentai`: eine testbare Kernbibliothek, ein CLI für
Automatisierung und Selbsttests, explizite Fehlerberichte sowie ein
reproduzierbarer Installations- und Endanwender-Testzyklus. Die
domänenspezifischen Funktionen bleiben getrennt: DocumentAI verarbeitet
Dokumente, UniversalRecovery verarbeitet Datenträger und Dateisysteme.

## Komponenten

| Komponente | Aufgabe |
|---|---|
| `ur-core` | Modelle, Fehler, I/O-Abstraktion, Dateisysteme und Recovery |
| `ur-cli` | Skriptbare Befehle, JSON-Ausgaben und Selbstdiagnose |
| `ur-recovery` | Wiederverwendbare High-Level-Recovery-Helfer |
| `ur-device-broker` | Plattformgrenze für privilegierten Gerätezugriff |

## Entwicklungsstatus

Der Rust-Workspace befindet sich im Aufbau. Dateimodelle, Fehler-Taxonomie,
Cache, Dateisystem-Erkennung und CLI-Selbsttest sind das Fundament; Parser,
Recovery-Strategien, GUI, Installer und Windows-Endanwenderlauf folgen
stufenweise.

## Lokale Prüfung

Mit installierter Rust-Toolchain:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo run -p ur-cli -- selftest
cargo run -p ur-cli -- scan-image .\tests\fixtures\sample.img
```

`selftest` verändert keine Datenträger und dient als schneller Smoke-Test für
die lokale Installation. `scan-image` akzeptiert ausschließlich eine reguläre
Datei und führt einen konservativen Header-Scan durch; physische Datenträger
werden von diesem Befehl nicht geöffnet.
