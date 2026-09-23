UniversalRecovery - Installations- und Bedienungsanleitung
============================================================

Status
------
Dieses Paket ist die aktuelle Rust-Core-Entwicklerversion. Es enthält den
Offline-Kern, Parser-Grundlagen, den sicheren Image-Scan und die Buildskripte.
Es enthält noch kein signiertes Fertigprogramm und darf nicht als fertige
Produktiv-Datenrettung beworben werden.

Sicherheitsregeln
-----------------
1. Niemals auf dem Originaldatenträger schreiben.
2. Zuerst ein sektorweises Abbild erstellen und dieses Abbild prüfen.
3. Wiederhergestellte Dateien immer auf ein anderes Laufwerk exportieren.
4. `scan-image` öffnet nur eine reguläre Image-Datei, niemals ein physisches
   Laufwerk.

Voraussetzungen für einen Build
-------------------------------
- Windows 10 oder Windows 11, 64 Bit
- Visual Studio 2022 mit "Desktopentwicklung mit C++"
- Rustup mit der stable-x86_64-pc-windows-msvc Toolchain

Build
-----
PowerShell als normaler Benutzer im Paketordner öffnen:

  .\installer\build.ps1

Das Skript führt Formatprüfung, Workspace-Check, Tests und Release-Build aus.
Bei fehlenden Buildtools bricht es mit einer verständlichen Fehlermeldung ab.

Installation
------------
Nach erfolgreichem Build:

  .\installer\install.ps1

Standardziel ist:

  %LOCALAPPDATA%\UniversalRecovery

Mit `-InstallRoot` kann ein anderes Benutzerverzeichnis gewählt werden. Eine
Installation benötigt keine Administratorrechte. Das Skript kopiert nur das
bereits geprüfte `ur-cli.exe`; es kompiliert nicht heimlich während der
Installation.

Smoke-Test
----------
Nach der Installation:

  & "$env:LOCALAPPDATA\UniversalRecovery\ur-cli.exe" selftest

Ein Image sicher prüfen:

  & "$env:LOCALAPPDATA\UniversalRecovery\ur-cli.exe" scan-image `
      "D:\Images\disk.img"

Erwartete Ausgabe ist eine einzelne JSON-Zeile mit `"ok":true`. Ein echter
Endanwender-Release ist erst freigegeben, wenn Build, Tests, Signaturprüfung,
Installation und Windows-Endanwenderlauf erfolgreich dokumentiert wurden.
