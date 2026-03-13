# TuxTax

**Linux-native Steuererklärungssoftware für deutsche Steuerpflichtige.**

Open-Source-Alternative zu WISO Steuer, Lexware QuickSteuer Deluxe und SteuerSparErklärung. Alle Daten bleiben lokal — kein Cloud-Zwang, kein Tracking.

## Features (geplant)

- Einkommensteuer mit allen 7 Einkunftsarten (§ 2 EStG)
- Gewerbesteuer, Umsatzsteuer, EÜR für Selbständige
- ELSTER-Übertragung via ERiC-Bibliothek
- Interview-Modus und Formularansicht
- Steuerberechnung mit Echtzeit-Erstattungsvorschau
- Vorausgefüllte Steuererklärung (VaSt)
- Steuerbescheid-Prüfung mit Einspruchsunterstützung
- Belegmanager mit OCR
- Verschlüsselte lokale Datenbank (SQLCipher / AES-256)

## Tech Stack

| Komponente | Technologie |
|---|---|
| Framework | Tauri 2.0 |
| Backend | Rust |
| Frontend | TypeScript + React (Vite) |
| Datenbank | SQLite + SQLCipher (AES-256) |
| ELSTER | ERiC C-Library via Rust FFI |
| Tests | cargo test (Rust), Vitest (TypeScript) |

## Projektstruktur

```
src/                          # Frontend (React/TypeScript)
  api/                        # Tauri IPC-Aufrufe
  components/                 # UI-Komponenten
    interview/                # Interview-Modus
    forms/                    # Formularansicht
    dashboard/                # Steuerfall-Übersicht
    documents/                # Belegmanager
  stores/                     # State Management
  types/                      # TypeScript-Typen
  i18n/                       # Internationalisierung (de, en)

src-tauri/                    # Backend (Rust)
  src/
    commands/                 # Tauri-Commands (IPC)
    tax/                      # Steuerberechnungs-Engine
      tariff.rs               # ESt-Tarif (§ 32a EStG)
      solidarity.rs           # Solidaritätszuschlag
      church_tax.rs           # Kirchensteuer
      calculators/            # Spezialrechner (Pendlerpauschale, AfA, ...)
    elster/                   # ERiC-Integration
    db/                       # SQLite/SQLCipher-Datenbankschicht
    forms/                    # Formular-Definitionen
    interview/                # Interview-Logik
    services/                 # OCR, PDF, Import/Export
  migrations/                 # SQL-Migrationen
```

## Voraussetzungen

- [Rust](https://rustup.rs/) (>= 1.75)
- [Node.js](https://nodejs.org/) (>= 18)
- Tauri 2.0 Systemabhängigkeiten:

```bash
# Ubuntu / Debian
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev \
  build-essential curl wget file patchelf libssl-dev
```

## Entwicklung

```bash
# Dependencies installieren
npm install

# Entwicklungsserver starten (Frontend + Backend)
npm run tauri dev

# Release-Build erstellen
npm run tauri build

# Rust-Tests ausführen
cd src-tauri && cargo test

# TypeScript-Build prüfen
npm run build
```

**Hinweis für VS Code Snap-Nutzer:** Falls das Fenster nicht öffnet, im externen Terminal starten:

```bash
unset GTK_PATH GTK_EXE_PREFIX GIO_MODULE_DIR GTK_IM_MODULE_FILE LOCPATH GSETTINGS_SCHEMA_DIR
npm run tauri dev
```

## Architektur

```
┌─────────────────────────────────────────────┐
│              React Frontend                 │
│  (Interview, Formulare, Dashboard, Belege)  │
├──────────────── Tauri IPC ──────────────────┤
│              Rust Backend                   │
│  ┌───────────┐ ┌──────────┐ ┌────────────┐ │
│  │ Steuer-   │ │ ELSTER/  │ │ Datenbank  │ │
│  │ berechnung│ │ ERiC     │ │ SQLCipher  │ │
│  └───────────┘ └──────────┘ └────────────┘ │
└─────────────────────────────────────────────┘
```

## Roadmap

| Meilenstein | Beschreibung | Status |
|---|---|---|
| M0 | Projekt-Setup & Infrastruktur | In Arbeit |
| M1 | Einkommensteuer für Arbeitnehmer (MVP) | Ausstehend |
| M2 | Erweiterte Einkunftsarten | Ausstehend |
| M3 | Selbständige & Gewerbe | Ausstehend |
| M4 | Feststellung & Sonderfälle | Ausstehend |
| M5 | Belegmanagement & UX-Polish | Ausstehend |
| M6 | Langfristige Features | Ausstehend |

Detaillierter Plan: [docs/PROJECT_PLAN.md](docs/PROJECT_PLAN.md)

## Dokumentation

- [Software-Spezifikation](docs/SPECIFICATION.md) — Requirements & Architektur
- [Projektplan](docs/PROJECT_PLAN.md) — Implementierungsplan (6 Meilensteine)
- [Wettbewerbsanalyse](COMPETITIVE_ANALYSIS.md) — Feature-Vergleich

## ELSTER / ERiC

Die ERiC-Bibliothek (`libericapi.so`) ist proprietäre Software der Finanzverwaltung und darf **nicht** redistribuiert werden. Nutzer müssen sie separat über das [ELSTER-Entwicklerportal](https://www.elster.de/elsterweb/entwickler) beziehen. TuxTax wird einen First-Run-Wizard enthalten, der beim Download und der Einrichtung unterstützt.

## Lizenz

TBD

## Mitwirken

Das Projekt befindet sich in einem frühen Stadium. Issues und Pull Requests sind willkommen.
