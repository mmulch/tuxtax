# TuxTax - Software-Spezifikation

## 1. Produktvision

TuxTax ist eine umfassende, Linux-native Steuererklarungssoftware fur deutsche Steuerpflichtige. Sie soll funktional mit WISO Steuer, Lexware QuickSteuer Deluxe und SteuerSparErklarung konkurrieren -- als Open-Source-Alternative, die primär unter Linux lauft, aber auch unter Windows und macOS nutzbar ist.

**Kernprinzipien:**
- Linux-first, cross-platform-fahig
- Datenschutz: Alle Daten lokal, kein Cloud-Zwang
- Open Source (Lizenz: TBD, ERiC-Bibliothek ist proprietar und darf nicht redistribuiert werden)
- Modular und erweiterbar

---

## 2. Funktionale Anforderungen

### 2.1 Steuerarten

#### Phase 1: Einkommensteuer (Kernprodukt)
- **Einkommensteuer (ESt)** inkl. Solidaritatszuschlag und Kirchensteuer
- Alle 7 Einkunftsarten nach Par. 2 EStG:
  1. Land- und Forstwirtschaft (Anlage L, 13a)
  2. Gewerbebetrieb (Anlage G)
  3. Selbstandige Arbeit (Anlage S)
  4. Nichtselbstandige Arbeit (Anlage N, N-AUS, N-GRE, N-DHH)
  5. Kapitalvermogen (Anlage KAP, KAP-BET, KAP-INV)
  6. Vermietung und Verpachtung (Anlage V, V-FeWo)
  7. Sonstige Einkuenfte (Anlage SO, R, R-AUS, R-AV/bAV)
- Sonderausgaben, Vorsorgeaufwand, aussergewohnliche Belastungen
- Anlage Kind, Anlage U (Unterhalt), Anlage K
- Anlage AV (Riester), Anlage AUS
- Anlage FW, Anlage Energetische Massnahmen (35c)
- Anlage 34a, Anlage Zinsschranke
- WA-ESt (weitere Angaben)
- Lohnsteuer-Ermassigung

#### Phase 2: Gewerbe & Selbstandige
- **Einnahmen-Uberschuss-Rechnung (EUR)** mit AVEUR
- **Umsatzsteuer-Voranmeldung (UStVA)** monatlich/vierteljährlich
- **Umsatzsteuer-Jahreserklarung (USt)**
- **Gewerbesteuererklarung (GewSt 1 A)** mit Zerlegung (GewSt 1 D)
- **Lohnsteuer-Anmeldung (LStA)**
- Dauerfristverlangerung
- Zusammenfassende Meldung (ZM)

#### Phase 3: Feststellung & Spezial
- **Gesonderte/einheitliche Feststellung (ESt 1 B, ESt 1 D)**
- Feststellungsbeteiligte (FB), FE-Anlagen 1-5
- Anlage SE (Sonderbetriebsausgaben), Anlage ER (Erganzungsrechnung)
- **Kapitalertragsteuer-Anmeldung**
- **Fragebogen zur steuerlichen Erfassung** (Gewerbeanmeldung etc.)

#### Phase 4: Weitere Steuerarten (Optional/Langfristig)
- Korperschaftsteuer (KSt)
- Grundsteuer (landerspezifisch)
- Erbschaft-/Schenkungsteuer
- E-Bilanz

### 2.2 Steuerberechnung

Die Software muss die vollstandige ESt-Berechnungskette implementieren:

1. Summe der Einkuenfte (alle 7 Einkunftsarten)
2. ./. Altersentlastungsbetrag, Entlastungsbetrag Alleinerziehende
3. = Gesamtbetrag der Einkuenfte
4. ./. Verlustabzug (Par. 10d EStG)
5. ./. Sonderausgaben (Par. 10-10c EStG)
6. ./. Aussergewohnliche Belastungen (Par. 33-33b EStG)
7. = Einkommen
8. ./. Kinderfreibetrage (Guenstigerpruefung vs. Kindergeld)
9. = Zu versteuerndes Einkommen (zvE)
10. Tarifberechnung (Par. 32a EStG) mit aktuellen Steuertarifen
11. + Solidaritatszuschlag (5,5% mit Freigrenze seit 2021)
12. + Kirchensteuer (8% oder 9% je Bundesland)

**Weitere Berechnungen:**
- Gewerbesteuer: Messbetrag (3,5%) x Hebesatz, Anrechnung nach Par. 35 EStG (Faktor 4,0)
- Umsatzsteuer: 19%/7%-Berechnung, Vorsteuerabzug, Kleinunternehmerregelung
- Progressionsvorbehalt
- Splittingtarif / Grundtarif
- Funftelregelung (Abfindungen etc.)

### 2.3 ELSTER/ERiC-Integration

#### Pflichtanforderungen
- Integration der ERiC C-Bibliothek (libericapi.so) uber FFI
- XML-Generierung nach ELSTER-Schnittstellenbeschreibung
- Authentifizierung uber Zertifikatsdatei (.pfx) mit PIN
- Datenubertragung an ELSTER-Clearingstelle
- Verarbeitung der Server-Ruckmeldungen
- PDF-Erzeugung uber ERiC-Druckfunktion
- Unterstuetzung der Datenarten: ESt, EUER, UStVA, USt, GewSt, LStA, FEIN, ZMDO

#### Erweiterte ELSTER-Funktionen
- **Vorausgefuellte Steuererklarung (VaSt)**: Abruf der beim Finanzamt vorliegenden Daten
- **Steuerbescheid-Abruf**: Elektronischer Steuerbescheid
- **Statusverfolgung**: Bearbeitungsstand der Erklarung
- **Personalausweis (nPA)**: eID-Authentifizierung uber OTTER-Protokoll
- **Sicherheitsstick**: Hardware-Token-Unterstuetzung

#### Technische ERiC-Details
- `EricInitialisiere()` / `EricBeende()` fur Lifecycle
- `EricBearbeiteVorgang()` fur Validierung und Ubertragung
- `EricCreateKey()` / `EricGetHandleToCertificate()` fur Zertifikatverwaltung
- Plugin-Laden: `libcheckESt_<jahr>.so`, `libcheckEUER_<jahr>.so` etc.
- Encoding: UTF-8 (kein BOM), Dezimaltrennzeichen: Punkt (.)
- Multithreading-API (`ericmtapi.h`) fur parallele Verarbeitung

### 2.4 Benutzeroberflache

#### Interview-Modus (Primarer Eingabemodus)
- Gefuhrter Frage-Antwort-Dialog, der schrittweise durch die Erklarung fuhrt
- Abhangigkeitsbasierte Navigation: nur relevante Fragen werden gestellt
- Kontext-sensitive Hilfetexte und Steuertipps
- Fortschrittsanzeige pro Themengebiet
- Zusammenfassung und Plausibilitatsprufung vor Absendung

#### Formularansicht (Alternativer Eingabemodus)
- Direkteingabe in Steuerformular-Darstellung (wie ELSTER Online)
- 1:1-Abbildung der amtlichen Formulare
- Umschaltbar zwischen Interview und Formular

#### Steuer-Dashboard
- Erstattungsvorschau / Nachzahlungsvorschau in Echtzeit
- Ubersicht aller Einkunftsarten mit Statusanzeige (vollstandig/unvollstandig)
- Vergleich mit Vorjahr
- Steueroptimierungshinweise

#### Steuerbescheid-Prufung
- Vergleich des erhaltenen Steuerbescheids mit eigener Berechnung
- Detaillierte Abweichungsanzeige pro Position
- Einspruchsunterstutzung: Muster-Einspruchsschreiben generieren
- Fristen-Management (Einspruchsfrist 1 Monat nach Zugang)

#### Weitere UI-Features
- Mehrere Steuerfalle pro Installation (min. 5)
- Vorjahresimport (eigene Daten aus TuxTax-Vorjahr)
- Dunkelmodus / System-Theme-Integration
- Mehrsprachig: Deutsch (primar), Englisch (sekundar)
- Tastaturnavigation und Barrierefreiheit (a11y)

### 2.5 Dokumentenmanagement

- **Belegmanager**: Belege digitalisieren, kategorisieren, Steuerpositionen zuordnen
- **OCR-Integration**: Texterkennung aus Fotos/Scans (Rechnungen, Bescheinigungen)
- **Importformate**: PDF, JPG, PNG, TIFF
- **Exportformate**: PDF (Erklarung), CSV (Daten), TuxTax-Projektdatei

### 2.6 Datenimport

- **VaSt-Import**: Vorausgefuellte Steuererklarung uber ELSTER
- **Vorjahresimport**: Stammdaten und wiederkehrende Posten aus Vorjahr
- **CSV/Banking-Import**: Kontoumsatze fur EUR
- **Krypto-Import**: CSV-Import fur Kryptowahrungstransaktionen (CoinTracking-Format etc.)

### 2.7 Spezialrechner

- Steuererstattungsrechner (Schnellberechnung)
- Steuerklassen-Optimierung fur Ehepaare
- Pendlerpauschale / Entfernungspauschale
- Riester-Zulagen-Rechner
- Abfindungsrechner (Funftelregelung)
- Kfz-Steuer-Rechner
- Photovoltaik-Steuermodul
- Abschreibungsrechner (AfA)

---

## 3. Nicht-funktionale Anforderungen

### 3.1 Sicherheit
- Verschlusselte lokale Datenspeicherung (SQLCipher: AES-256)
- Kein Klartextspeichern von PINs/Passworten
- Sichere Handhabung von Zertifikatsdateien (.pfx)
- Keine Telemetrie, keine Cloud-Ubertragung ohne explizite Zustimmung
- TLS fur alle ELSTER-Verbindungen (uber ERiC)
- Sichere Loschung temporarer Steuerdaten

### 3.2 Performance
- Starten in < 3 Sekunden
- Steuerberechnung in < 1 Sekunde
- ELSTER-Ubertragung: abhangig von Netzwerk, aber Timeout-Handling
- Speicherbedarf: < 500 MB RAM

### 3.3 Plattform-Unterstutzung
- **Primar**: Linux x86_64 (Debian 12+, Ubuntu 22.04+, Fedora 39+, openSUSE Leap 15.5+)
- **Sekundar**: Windows 10/11 (64-bit), macOS 12+ (Intel + ARM)
- Packaging: Flatpak (primar), DEB, RPM, AppImage, AUR

### 3.4 Datenformat
- Offenes Projektformat (SQLite-Datenbank pro Steuerfall)
- XML fur ELSTER-Daten (per ERiC-Spezifikation)
- Backup/Restore-Funktionalitat
- Daten-Export in menschen-lesbare Formate (PDF, CSV)

---

## 4. Software-Architektur

### 4.1 Technologie-Stack

| Schicht | Technologie | Begrundung |
|---|---|---|
| **Sprache (Backend)** | Rust | Systemnahe Sprache, sichere FFI zu ERiC (C), Memory Safety, existierende Bindings (eric-rs), Cross-Platform |
| **GUI-Framework** | Tauri 2.0 | Rust-Backend + Web-Frontend, kleine Binaries, sichere IPC, native Systemintegration |
| **Frontend** | TypeScript + React | Grosse Community, Komponenten-Okosystem, schnelle UI-Entwicklung |
| **Datenbank** | SQLite + SQLCipher | Lokale Einzeldatei-DB, AES-256-Verschlusselung, kein Server notig |
| **ERiC-Integration** | Rust FFI (eric-rs) | Direkte C-Library-Anbindung ohne Overhead |
| **PDF-Erzeugung** | ERiC (nativ) + printpdf | ERiC fur amtliche PDFs, printpdf fur Berichte |
| **OCR** | Tesseract (per CLI/Binding) | Open-Source-OCR fur Belegerfassung |
| **Testing** | Rust: cargo test, TS: Vitest | Native Testframeworks |

### 4.2 Architektur-Ubersicht

```
+------------------------------------------------------------------+
|                        Tauri Application                          |
|  +-----------------------------+  +----------------------------+  |
|  |     Frontend (WebView)      |  |     Rust Backend           |  |
|  |                             |  |                            |  |
|  |  +---------------------+   |  |  +----------------------+  |  |
|  |  | Interview-Engine    |   |  |  | Steuerberechnung     |  |  |
|  |  | (React + TS)        |   |  |  | (Tax Calculation)    |  |  |
|  |  +---------------------+   |  |  +----------------------+  |  |
|  |  +---------------------+   |  |  +----------------------+  |  |
|  |  | Formular-Renderer   |<---->| | ELSTER-Modul         |  |  |
|  |  | (React + TS)        |   |  |  | (ERiC FFI via        |  |  |
|  |  +---------------------+   |  |  |  eric-rs)            |  |  |
|  |  +---------------------+   |  |  +----------------------+  |  |
|  |  | Dashboard /         |   |  |  +----------------------+  |  |
|  |  | Visualisierung      |   |  |  | Datenhaltung         |  |  |
|  |  +---------------------+   |  |  | (SQLCipher)          |  |  |
|  |  +---------------------+   |  |  +----------------------+  |  |
|  |  | Belegmanager        |   |  |  +----------------------+  |  |
|  |  +---------------------+   |  |  | Dokumenten-Service   |  |  |
|  |                             |  |  | (PDF, OCR, Import)   |  |  |
|  +-----------------------------+  |  +----------------------+  |  |
|                                   +----------------------------+  |
+------------------------------------------------------------------+
                              |
                    +-------------------+
                    |  ERiC C-Library   |
                    |  (libericapi.so)  |
                    +-------------------+
                              |
                    +-------------------+
                    | ELSTER-Server     |
                    | (Clearingstelle)  |
                    +-------------------+
```

### 4.3 Modulstruktur (Rust Backend)

```
tuxtax/
  src-tauri/
    src/
      main.rs                    # Tauri-Einstiegspunkt
      lib.rs                     # Modul-Registrierung
      commands/                  # Tauri-Commands (IPC Frontend<->Backend)
        mod.rs
        tax_case.rs              # CRUD Steuerfalle
        calculation.rs           # Steuerberechnung auslosen
        elster.rs                # ELSTER-Operationen
        documents.rs             # Belegmanagement
        import_export.rs         # Datenimport/-export
      tax/                       # Steuerberechnungs-Engine
        mod.rs
        income.rs                # Einkunftsarten
        deductions.rs            # Abzuge (Sonderausgaben, agB etc.)
        tariff.rs                # Steuertarif (Par. 32a EStG)
        solidarity.rs            # Solidaritatszuschlag
        church_tax.rs            # Kirchensteuer
        trade_tax.rs             # Gewerbesteuer
        vat.rs                   # Umsatzsteuer
        child_benefit.rs         # Kindergeld/Kinderfreibetrag-Guenstigerpruefung
        splitting.rs             # Ehegattensplitting
        calculators/             # Spezialrechner
          commuter.rs            # Pendlerpauschale
          depreciation.rs        # AfA
          severance.rs           # Abfindung/Funftelregelung
          riester.rs             # Riester-Zulagen
      elster/                    # ELSTER/ERiC-Integration
        mod.rs
        eric_ffi.rs              # FFI-Bindings (via eric-rs)
        xml_builder.rs           # XML-Generierung nach ELSTER-Schema
        xml_parser.rs            # Server-Antwort parsen
        certificate.rs           # Zertifikatverwaltung
        vast.rs                  # Vorausgefuellte Steuererklarung
        submission.rs            # Ubertragungslogik
      db/                        # Datenbankschicht
        mod.rs
        schema.rs                # SQLite-Schema
        migrations.rs            # Schema-Migrationen
        tax_case.rs              # Steuerfall-CRUD
        documents.rs             # Beleg-Speicherung
      forms/                     # Formular-Definitionen
        mod.rs
        est/                     # Einkommensteuer-Formulare
          mantelbogen.rs
          anlage_n.rs
          anlage_v.rs
          anlage_kap.rs
          ...
        eur.rs                   # EUR
        ustva.rs                 # UStVA
        gewst.rs                 # GewSt
      interview/                 # Interview-Logik (Regelwerk)
        mod.rs
        rules.rs                 # Abhangigkeiten zwischen Fragen
        questions.rs             # Fragenkatalog
      services/                  # Querschnittsdienste
        mod.rs
        ocr.rs                   # Tesseract-Integration
        pdf.rs                   # PDF-Erzeugung
        crypto_import.rs         # Krypto-CSV-Import
        banking_import.rs        # Banking-CSV-Import
  src/                           # Frontend (React/TypeScript)
    App.tsx
    components/
      interview/                 # Interview-UI-Komponenten
      forms/                     # Formular-UI-Komponenten
      dashboard/                 # Dashboard-Komponenten
      documents/                 # Belegmanager-UI
    stores/                      # State Management (Zustand o.a.)
    api/                         # Tauri IPC-Aufrufe
    types/                       # TypeScript-Typen
    i18n/                        # Internationalisierung (de, en)
```

### 4.4 Datenmodell (Kernentitaten)

```
Steuerfall (TaxCase)
  ├── id, name, steuerjahr, status
  ├── steuerpflichtiger (Person)
  │     ├── name, geburtsdatum, steuer_id, religion, familienstand
  │     └── adresse, finanzamt, steuernummer
  ├── ehepartner (Person, optional)
  ├── kinder[] (Kind)
  │     ├── name, geburtsdatum, steuer_id
  │     └── kindergeld_empfanger, betreuungskosten
  ├── einkuenfte[]
  │     ├── art (N, V, KAP, G, S, R, SO, L)
  │     └── typ-spezifische Felder
  ├── abzuege[]
  │     ├── sonderausgaben, vorsorgeaufwand
  │     ├── aussergewoehnliche_belastungen
  │     └── handwerkerleistungen, haushaltsnahe_dl
  ├── belege[] (Dokument)
  │     ├── datei, kategorie, betrag, zuordnung
  │     └── ocr_text, thumbnail
  ├── berechnung (Ergebnis)
  │     ├── zu_versteuerndes_einkommen
  │     ├── einkommensteuer, soli, kirchensteuer
  │     └── erstattung_oder_nachzahlung
  └── elster_status
        ├── uebertragen_am, transferticket
        ├── bescheid_eingegangen
        └── bescheid_abweichungen[]
```

---

## 5. Abgrenzungen & Einschrankungen

### Was TuxTax NICHT ist (zumindest initial)
- Keine Finanzbuchhaltungssoftware (kein Ersatz fur Lexware Office, DATEV etc.)
- Keine Lohnbuchhaltungssoftware
- Kein Steuerberater-Werkzeug (kein Mandantenmanagement)
- Keine E-Bilanz-Erstellung (erfordert XBRL-Spezialwissen)

### ERiC-Einschrankungen
- Die ERiC-Bibliothek darf **nicht redistribuiert** werden -- Nutzer mussen sie separat beziehen oder TuxTax ladt sie beim ersten Start herunter (mit Lizenz-Akzeptanz)
- ERiC-Updates erscheinen zweimal jahrlich (Mai + November) -- TuxTax muss kompatibel bleiben
- Testbetrieb nur mit Testfinanzamtern und Teststeuernummern moglich
- Hersteller-ID muss bei der ELSTER-Entwicklerregistrierung beantragt werden

### Rechtliche Hinweise
- Keine Rechts- oder Steuerberatung -- Software ist Hilfsmittel, kein Ersatz fur Steuerberater
- Keine Garantie fur Korrektheit der Steuerberechnung
- ERiC fuhrt eigene Plausibilitatsprufung durch, die aber nicht alle Fehler abfangt
