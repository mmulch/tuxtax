# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

TuxTax — umfassende Linux-native Steuererklarungssoftware fur deutsche Steuerpflichtige. Konkurriert mit WISO Steuer, Lexware QuickSteuer Deluxe, SteuerSparErklarung. Open Source, Datenschutz-fokussiert (alle Daten lokal).

## Repository

- GitHub: https://github.com/mmulch/tuxtax
- Branch: main
- Maintainer: mmulch <marcel@mulch-online.de>

## Tech Stack

- **Backend**: Rust (Tauri 2.0)
- **Frontend**: TypeScript + React (Vite)
- **Database**: SQLite + SQLCipher (AES-256 verschlusselt)
- **ELSTER-Integration**: ERiC C-Library via Rust FFI (eric-rs crate)
- **Testing**: cargo test (Rust), Vitest (TypeScript)
- **Linting**: clippy (Rust), eslint (TypeScript)

## Architecture

- Tauri-App: Rust-Backend kommuniziert uber IPC-Commands mit React-Frontend
- ERiC (libericapi.so): Proprietare C-Bibliothek der Finanzverwaltung fur ELSTER-Ubertragung -- darf nicht redistribuiert werden
- Steuerberechnung: Eigene Engine in Rust (Tarif nach Par. 32a EStG, Soli, KiSt, GewSt, USt)
- Datenformat: SQLite-DB pro Steuerfall, ELSTER-XML nach offiziellen Schemas
- Formular-Definitionen in Rust, Interview-Logik abhangigkeitsbasiert

## Key Documentation

- `docs/SPECIFICATION.md` — Software-Spezifikation (Requirements & Architektur)
- `docs/PROJECT_PLAN.md` — Implementierungsplan (6 Meilensteine)
- `COMPETITIVE_ANALYSIS.md` — Feature-Vergleich mit Konkurrenzprodukten

## Important Conventions

- Alle Steuertarife/Freibetrage datengetrieben (Konfiguration pro Steuerjahr), nicht hardcoded
- ERiC-Bibliothek wird hinter einer Abstraktionsschicht gekapselt (Modul `elster/`)
- XML-Encoding: UTF-8 ohne BOM, Dezimaltrennzeichen: Punkt (.)
- Sprache im Code: Englisch. Sprache in UI/Docs: Deutsch (primar), Englisch (sekundar)
