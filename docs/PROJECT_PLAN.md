# TuxTax - Projektplan

## Ubersicht

Der Projektplan ist in 6 Meilensteine unterteilt. Jeder Meilenstein liefert ein funktionsfahiges Inkrement. Die geschatzte Reihenfolge priorisiert ein fruhes lauffähiges Produkt (ESt fur Arbeitnehmer) und erweitert schrittweise.

---

## Meilenstein 0: Projekt-Setup & Infrastruktur

### Ziele
- Entwicklungsumgebung und CI/CD aufsetzen
- ELSTER-Entwicklerzugang beantragen
- Technologie-Stack validieren

### Aufgaben

#### 0.1 ELSTER-Entwicklerregistrierung
- [ ] Registrierung auf https://www.elster.de/elsterweb/registrierung-entwickler/form
- [ ] Prufung durch BayLfSt abwarten (kann mehrere Wochen dauern)
- [ ] Zugangsdaten erhalten, ERiC-SDK herunterladen
- [ ] Hersteller-ID beantragen
- [ ] Teststeuernummern und Testfinanzamter einrichten
- **KRITISCH**: Ohne Hersteller-ID keine produktive ELSTER-Ubertragung moglich. Fruh starten!

#### 0.2 Projekt-Scaffolding
- [ ] Tauri 2.0 Projekt initialisieren (`cargo create-tauri-app`)
- [ ] Rust-Backend-Struktur anlegen (Module wie in Spezifikation)
- [ ] React/TypeScript-Frontend mit Vite aufsetzen
- [ ] SQLite/SQLCipher-Integration (rusqlite + sqlcipher-Feature)
- [ ] eric-rs Dependency einbinden und ERiC-Anbindung testen
- [ ] Grundlegendes Logging (env_logger oder tracing)

#### 0.3 CI/CD
- [ ] GitHub Actions: Build (Linux, Windows, macOS)
- [ ] GitHub Actions: Tests (cargo test + vitest)
- [ ] GitHub Actions: Linting (clippy + eslint)
- [ ] Release-Pipeline: Tauri-Bundling fur Flatpak, DEB, RPM, AppImage

#### 0.4 ERiC-Proof-of-Concept
- [ ] ERiC-Bibliothek laden und initialisieren (`EricInitialisiere()`)
- [ ] Einfache UStVA-XML generieren und gegen ERiC validieren
- [ ] Testubertragung an ELSTER-Testserver
- [ ] Server-Antwort empfangen und parsen
- **Ergebnis**: Nachweis, dass die ERiC-Integration auf Linux funktioniert

---

## Meilenstein 1: Einkommensteuer fur Arbeitnehmer (MVP)

**Zielgruppe**: Arbeitnehmer mit einfacher Steuererklarung (grosste Nutzergruppe)

### 1.1 Datenbank & Steuerfall-Verwaltung
- [ ] SQLite-Schema definieren (Steuerfall, Person, Einkuenfte, Abzuge)
- [ ] Schema-Migrationen (refinery oder sqlx)
- [ ] CRUD-Operationen fur Steuerfalle
- [ ] Tauri-Commands: Steuerfall erstellen, offnen, loschen, exportieren

### 1.2 Stammdaten-Eingabe
- [ ] Personliche Daten (Name, Adresse, Geburtsdatum, Steuer-ID, Religion)
- [ ] Familienstand, Ehepartner-Daten
- [ ] Finanzamt-Auswahl (Datenbank aller Finanzamter mit BuFa-Nummern)
- [ ] Steuernummer-Eingabe mit Validierung (EricPruefeSteuernummer)
- [ ] Bankverbindung

### 1.3 Anlage N (Nichtselbstandige Arbeit)
- [ ] Arbeitslohn lt. Lohnsteuerbescheinigung
- [ ] Werbungskosten (Arbeitsmittel, Fortbildung, Bewerbungskosten etc.)
- [ ] Pendlerpauschale / Entfernungspauschale-Rechner
- [ ] Doppelte Haushaltsfuhrung (Anlage N-DHH)
- [ ] Homeoffice-Pauschale

### 1.4 Sonderausgaben & Vorsorge
- [ ] Anlage Vorsorgeaufwand (Kranken-/Pflegeversicherung, Altersvorsorge)
- [ ] Sonderausgaben (Spenden, Kirchensteuer, Ausbildungskosten)
- [ ] Anlage AV (Riester-Beitrage)

### 1.5 Aussergewohnliche Belastungen & Haushaltsnahe DL
- [ ] Krankheitskosten, Behinderung, Pflege
- [ ] Handwerkerleistungen (Par. 35a EStG)
- [ ] Haushaltsnahe Dienstleistungen
- [ ] Energetische Massnahmen (Anlage 35c)

### 1.6 Kinder
- [ ] Anlage Kind (pro Kind)
- [ ] Kinderfreibetrag vs. Kindergeld-Gunstigerpruefung
- [ ] Kinderbetreuungskosten
- [ ] Schulgeld, Ausbildungsfreibetrag

### 1.7 Steuerberechnung (Kern-Engine)
- [ ] ESt-Tarif nach Par. 32a EStG (alle Zonen, Formeln)
- [ ] Grundtarif / Splittingtarif
- [ ] Solidaritatszuschlag (mit Freigrenze/Milderungszone)
- [ ] Kirchensteuer (8% BaWu/Bayern, 9% ubrige Lander)
- [ ] Gunstigerpruefung Kindergeld/Kinderfreibetrag
- [ ] Erstattung/Nachzahlung berechnen (unter Berucksichtigung bereits gezahlter LSt)
- [ ] Unit-Tests mit realen Steuerfall-Beispielen

### 1.8 Interview-Modus (Basis)
- [ ] Fragenstruktur-Engine (abhangigkeitsbasiert)
- [ ] Erste Fragenstrecke: Arbeitnehmer-Einkommensteuer
- [ ] Navigationsleiste mit Fortschrittsanzeige
- [ ] Kontexthilfe-Texte pro Feld

### 1.9 ELSTER-Ubertragung (ESt)
- [ ] XML-Generierung fur ESt (Mantelbogen + Anlagen N, Vorsorgeaufwand, Kind etc.)
- [ ] Validierung uber ERiC
- [ ] Authentifizierte Ubertragung (.pfx-Zertifikat)
- [ ] Transferticket speichern
- [ ] PDF-Ausdruck uber ERiC

### 1.10 Dashboard
- [ ] Steuerfall-Ubersicht (Liste aller Falle)
- [ ] Echtzeit-Erstattungsvorschau wahrend der Eingabe
- [ ] Statusanzeige (Entwurf / Validiert / Ubertragen / Bescheid erhalten)

---

## Meilenstein 2: Erweiterte Einkunftsarten

### 2.1 Anlage KAP (Kapitalertrage)
- [ ] Kapitalertrage, Verlustverrechnung
- [ ] Gunstigerpruefung (Abgeltungsteuer vs. personlicher Steuersatz)
- [ ] Anlage KAP-BET, KAP-INV

### 2.2 Anlage V (Vermietung und Verpachtung)
- [ ] Mieteinnahmen, Werbungskosten bei Vermietung
- [ ] AfA-Rechner fur Immobilien (linear/degressiv)
- [ ] Anlage V-FeWo (Ferienwohnungen)

### 2.3 Anlage R (Renten)
- [ ] Gesetzliche Rente (Besteuerungsanteil nach Rentenbeginn)
- [ ] Private Renten, Riester-Renten
- [ ] Anlage R-AUS (auslandische Renten)
- [ ] Anlage R-AV/bAV

### 2.4 Anlage SO (Sonstige Einkuenfte)
- [ ] Private Verausserungsgeschafte (Immobilien < 10 Jahre, Krypto < 1 Jahr)
- [ ] Unterhaltszahlungen als Einkuenfte
- [ ] Krypto-Import (CoinTracking-CSV, Blockpit etc.)

### 2.5 VaSt-Import
- [ ] Vorausgefuellte Steuererklarung uber ELSTER abrufen
- [ ] Automatische Befuellung der Felder
- [ ] Diff-Ansicht (VaSt-Daten vs. eigene Eingaben)

### 2.6 Vorjahresimport
- [ ] Stammdaten aus Vorjahres-Steuerfall ubernehmen
- [ ] Wiederkehrende Posten vorschlagen (Versicherungen, Werbungskosten etc.)

### 2.7 Steuerbescheid-Prufung
- [ ] Elektronischen Steuerbescheid uber ELSTER abrufen
- [ ] Vergleich mit eigener Berechnung pro Position
- [ ] Abweichungsanzeige mit Erlauterung
- [ ] Einspruchsfrist-Tracking

---

## Meilenstein 3: Selbstandige & Gewerbe

### 3.1 Anlage G & Anlage S
- [ ] Gewerbebetrieb und selbstandige Tatigkeit
- [ ] Betriebseinnahmen/-ausgaben Ubersicht

### 3.2 Einnahmen-Uberschuss-Rechnung (EUR)
- [ ] EUR-Formular mit allen Positionen
- [ ] AVEUR (Anlageverzeichnis)
- [ ] AfA-Berechnung fur Betriebsvermogen
- [ ] CSV-Import fur Geschaftsvorfalle
- [ ] Kontenrahmen SKR 03/04 Zuordnung

### 3.3 Umsatzsteuer
- [ ] UStVA (Voranmeldung) monatlich/vierteljährlich
- [ ] USt-Jahreserklarung
- [ ] Dauerfristverlangerung
- [ ] Zusammenfassende Meldung (ZM)
- [ ] Kleinunternehmerregelung (Par. 19 UStG)
- [ ] Vorsteuerabzug

### 3.4 Gewerbesteuer
- [ ] GewSt 1 A
- [ ] Hinzurechnungen / Kurzungen
- [ ] Messbetrag-Berechnung
- [ ] Hebesatz-Datenbank (alle Gemeinden)
- [ ] Anrechnung auf ESt (Par. 35 EStG)
- [ ] GewSt-Zerlegung (GewSt 1 D)

### 3.5 Lohnsteuer-Anmeldung
- [ ] LStA-Formular
- [ ] Monatliche/vierteljährliche/jahrliche Anmeldung

### 3.6 Photovoltaik-Modul
- [ ] Steuerliche Behandlung kleiner PV-Anlagen (Par. 3 Nr. 72 EStG)
- [ ] Liebhaberei-Prufung
- [ ] USt-Optionen (Regelbesteuerung vs. Kleinunternehmer)

---

## Meilenstein 4: Feststellung & Sonderfalle

### 4.1 Feststellungserklarungen
- [ ] Gesonderte Feststellung (ESt 1 B)
- [ ] Einheitliche Feststellung (ESt 1 D)
- [ ] Feststellungsbeteiligte (FB)
- [ ] FE-Anlagen 1-5
- [ ] Anlage SE, Anlage ER

### 4.2 Anlage L (Land- und Forstwirtschaft)
- [ ] Grundformular
- [ ] Anlage 13a (Durchschnittssatzbesteuerung)

### 4.3 Anlage AUS (Auslandische Einkuenfte)
- [ ] DBA-Berucksichtigung
- [ ] Anrechnungsverfahren / Freistellungsverfahren
- [ ] Progressionsvorbehalt

### 4.4 Weitere Anlagen
- [ ] Anlage 34a (Thesaurierungsbeguenstigung)
- [ ] Anlage Zinsschranke
- [ ] Anlage FW (Forderung Wohneigentum)
- [ ] Antrag auf Nichtveranlagung (NV)

### 4.5 Lohnsteuer-Ermassigung
- [ ] Antrag auf Freibetrag
- [ ] Elektronische Ubermittlung

---

## Meilenstein 5: Belegmanagement & UX-Polish

### 5.1 Belegmanager
- [ ] Beleg-Upload (Drag & Drop, Dateiauswahl)
- [ ] OCR-Integration (Tesseract)
- [ ] Automatische Kategorisierung (Regelbasiert)
- [ ] Zuordnung zu Steuerpositionen
- [ ] Thumbnail-Vorschau

### 5.2 Steueroptimierung
- [ ] Optimierungshinweise wahrend der Eingabe ("Tipp: Sie konnen X noch absetzen")
- [ ] Steuerklassen-Empfehlung fur Ehepaare
- [ ] Vergleich: Zusammenveranlagung vs. Einzelveranlagung
- [ ] Vorjahresvergleich (Was hat sich geandert?)

### 5.3 Einspruchsunterstutzung
- [ ] Muster-Einspruchsschreiben generieren
- [ ] Haufige Einspruchsgrunde-Datenbank
- [ ] Elektronischer Einspruch uber ELSTER (ElsterNachricht)

### 5.4 UX-Verbesserungen
- [ ] Dunkelmodus
- [ ] Barrierefreiheit (a11y Audit)
- [ ] Tastaturnavigation
- [ ] Mehrsprachigkeit (i18n: de, en)
- [ ] Onboarding-Tutorial fur Erstnutzer
- [ ] Kontextuelle Steuertipps-Datenbank (min. 500 Tipps)

### 5.5 Packaging & Distribution
- [ ] Flatpak (Flathub-Veroffentlichung)
- [ ] DEB-Paket (Debian, Ubuntu)
- [ ] RPM-Paket (Fedora, openSUSE)
- [ ] AppImage
- [ ] AUR (Arch Linux)
- [ ] Windows Installer (MSI/NSIS)
- [ ] macOS DMG
- [ ] Automatische Update-Prufung

---

## Meilenstein 6: Langfristig / Zukunft

### 6.1 Weitere Steuerarten
- [ ] Korperschaftsteuer (KSt)
- [ ] Grundsteuer (landerspezifisch -- 16 verschiedene Modelle)
- [ ] Erbschaft-/Schenkungsteuer
- [ ] Kapitalertragsteuer-Anmeldung
- [ ] Fragebogen zur steuerlichen Erfassung

### 6.2 nPA-Authentifizierung
- [ ] Personalausweis uber OTTER-Protokoll
- [ ] AusweisApp2-Integration
- [ ] NFC-Kartenleser-Unterstutzung

### 6.3 KI-Assistenz
- [ ] LLM-basierter Steuerassistent (lokal, z.B. llama.cpp)
- [ ] Natürlichsprachliche Fragen beantworten
- [ ] Automatische Belegkategorisierung per ML
- [ ] Steueroptimierungsvorschlage

### 6.4 Community & Okosystem
- [ ] Plugin-System fur Drittanbieter-Erweiterungen
- [ ] Import-Schnittstelle fur Banking-Software (z.B. Hibiscus/HBCI)
- [ ] Import von Konkurrenzprodukten (WISO, QuickSteuer Export-Formate)

---

## Empfohlene Vorgehensweise

### Sofort starten
1. **ELSTER-Entwicklerregistrierung** -- das kann Wochen dauern und blockiert die ERiC-Integration
2. **Projekt-Scaffolding** (Meilenstein 0.2) -- parallel zur Wartezeit auf ELSTER

### Entwicklungsreihenfolge
1. **M0**: Infrastruktur + ERiC-PoC (Fundament)
2. **M1**: Arbeitnehmer-MVP (grosste Zielgruppe, fruhes nutzbares Produkt)
3. **M2**: Erweiterte Einkunftsarten (Kapitalertrage, Vermietung, Renten)
4. **M3**: Selbstandige (erweitert Zielgruppe erheblich)
5. **M5**: UX-Polish + Belegmanager (parallel zu M3/M4 moglich)
6. **M4**: Feststellung & Sonderfalle (Nische)
7. **M6**: Langfristige Features

### Risiken
| Risiko | Auswirkung | Mitigation |
|---|---|---|
| ELSTER-Registrierung dauert lange | Blockiert ERiC-Tests | Fruh beantragen, parallel an UI/Berechnung arbeiten |
| ERiC-API-Anderungen | Kompatibilitatsprobleme | ERiC-Abstraktionsschicht, regelm. Updates verfolgen |
| Steuertarif-Anderungen jahrlich | Berechnungs-Updates notig | Tarife datengetrieben (nicht hardcoded), Konfigurationsdateien pro Jahr |
| Korrektheit der Steuerberechnung | Falsche Ergebnisse -> Vertrauensverlust | Umfangreiche Test-Suite mit realen Steuerfallen, Vergleich mit ELSTER-Plausibilitätsprufung |
| ERiC-Bibliothek nicht redistribuierbar | Komplizierte Installation fur Nutzer | Installer/First-Run-Wizard der ERiC automatisch herunterlädt |
