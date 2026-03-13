# German Tax Software Competitive Analysis & Feature Matrix

## Table of Contents
1. [Product Overview](#1-product-overview)
2. [Feature Matrix](#2-feature-matrix)
3. [Supported Tax Forms (Anlagen) Matrix](#3-supported-tax-forms-anlagen-matrix)
4. [Supported Tax Types (Steuerarten) Matrix](#4-supported-tax-types-steuerarten-matrix)
5. [ELSTER/ERiC Technical Integration](#5-elsteric-technical-integration)
6. [German Tax Law Fundamentals for Software](#6-german-tax-law-fundamentals-for-software)
7. [Sources](#7-sources)

---

## 1. Product Overview

### 1.1 Lexware QuickSteuer Deluxe 2026
- **Publisher**: Lexware (Haufe Group)
- **Platform**: Windows only (64-bit, Win 10/11)
- **Price**: ~32.90 EUR
- **Max declarations**: 5 per license (Deluxe), 3 (Standard)
- **Target users**: Employees, retirees, self-employed, small business owners
- **Stiftung Warentest**: 94% rating

### 1.2 WISO Steuer 2026
- **Publisher**: Buhl Data Service
- **Platform**: Windows, Mac, iOS, Android, Browser
- **Price**: ~22.90 EUR
- **Max declarations**: 5 per license
- **Target users**: All income types including self-employed
- **Stiftung Warentest**: 98% rating, Testsieger (test winner), Note 1.3 (sehr gut)

### 1.3 SteuerSparErklarung 2026
- **Publisher**: Wolters Kluwer / Akademische Arbeitsgemeinschaft (steuertipps.de)
- **Platform**: Windows, Mac
- **Price**: Flex ~36.95 EUR (3 returns), Plus ~46.95 EUR (5 returns), Selbststandige ~94.95 EUR (5 returns)
- **Max declarations**: 3-5 depending on variant
- **Target users**: Employees, retirees, self-employed (variant-dependent)
- **Stiftung Warentest**: 92% rating

### 1.4 tax 2026 (Buhl)
- **Publisher**: Buhl Data Service
- **Platform**: Windows only (64-bit, Win 10/11, min 4 GB RAM, 3 GB disk)
- **Price**: Standard ~15-20 EUR, Professional ~30-40 EUR
- **Max declarations**: Standard: 5, Professional: 15
- **Target users**: Standard = private individuals; Professional = freelancers, self-employed, entrepreneurs

### 1.5 Smartsteuer (Online)
- **Publisher**: Lexware/Haufe Group
- **Platform**: Browser-based (any OS, no installation)
- **Price**: ~39.99 EUR
- **Max declarations**: 5 per license
- **Target users**: Employees, retirees, self-employed, freelancers
- **Certification**: TUV Rheinland certified data security, SSL encryption

### 1.6 ELSTER Online (Mein ELSTER)
- **Publisher**: German Federal/State Tax Administration (Bayerisches Landesamt fur Steuern)
- **Platform**: Browser-based
- **Price**: Free
- **Max declarations**: Unlimited
- **Target users**: All taxpayers
- **Certification**: ISO 27001

---

## 2. Feature Matrix

| Feature | QuickSteuer Deluxe | WISO Steuer | SteuerSparErkl. | tax (Buhl) | Smartsteuer | ELSTER Online |
|---|---|---|---|---|---|---|
| **User Interface** |||||||
| Interview/Wizard mode | Yes | Yes | Yes (Q&A style) | Yes | Yes (step-by-step) | Partial (Anlage-Assistent) |
| AI assistant | No | Yes (SteuerGPT) | Yes (Alma) | No | No | No |
| Video tutorials | No | No | No | No | No | No |
| Tax tips (number) | Yes (tips included) | 1,000+ | 1,000+ (Plus version) | 1,000+ | Yes (integrated) | No |
| Plausibility check | Yes (Steuerprufer) | Yes (automatic) | Yes (automatic) | Yes (automatic) | Yes (smartCheck) | Yes (basic) |
| **Data Import** |||||||
| Prior-year data import | Yes | Yes | Yes | Yes | Yes | No |
| VaSt (pre-filled return) | Yes | Yes | Yes | Yes | Yes | Yes |
| Bank data import | No | Yes | No | Yes | No | No |
| Salary/pension auto-import | No | Yes | No | Yes | No | No |
| Crypto import (CoinTracking) | No | No | Yes (CoinTracking partner) | No | No | No |
| Lexware Office integration | No | No | No | No | Yes | No |
| **ELSTER Integration** |||||||
| ELSTER submission | Yes (ELSTER-Assistent) | Yes (digital signature) | Yes | Yes | Yes | Native |
| ELSTER certificate mgmt | Yes | Yes | Yes | Flex: without cert | Yes | Native |
| Status tracking | Yes (real-time) | Yes | Yes | Yes | Yes | Yes |
| Electronic Steuerbescheid | Yes | Yes | Yes | Yes | Yes | Yes |
| **Tax Assessment** |||||||
| Steuerbescheid-Prufung | Yes | Yes (detailed discrepancy view) | Yes | Yes | Yes | No |
| Einspruch (objection) support | No | Yes | Yes | Yes | No | No |
| Refund calculator/preview | Yes | Yes | Yes | Yes | Yes (free preview) | No |
| **Document Management** |||||||
| BelegManager / receipt mgmt | Yes (Deluxe only) | Yes (BelegManager App) | Yes (BelegManager, Plus+) | Yes (BelegManager App) | No | Yes (basic, since 2025) |
| Receipt scanning (OCR) | Yes | Yes | Yes (scan/photo import) | Yes | No | No |
| **Special Features** |||||||
| Photovoltaik module | No | No | Yes (Selbststandige ed.) | No | No | No |
| Grundsteuer calculator | No | Yes | No | No | No | Yes (state-specific) |
| Kfz-Steuer calculator | No | Yes | No | No | No | No |
| Riester calculator | No | Yes | No | No | No | No |
| Abfindung calculator | No | Yes | No | No | No | No |
| Steuerklassen-Empfehlung | No | No | Yes (Flex+) | No | No | No |
| **Platform** |||||||
| Windows | Yes | Yes | Yes | Yes | Yes (browser) | Yes (browser) |
| macOS | No | Yes | Yes | No | Yes (browser) | Yes (browser) |
| iOS/Android | No | Yes (native app) | No | No | Yes (browser) | Yes (MeinELSTER+ app, NRW pilot from Jul 2026) |
| Browser-based | No | Yes (WISO Steuer Web) | No | No | Yes | Yes |

---

## 3. Supported Tax Forms (Anlagen) Matrix

### 3.1 Einkommensteuer Forms (ESt 1 A)

| Anlage / Form | QuickSteuer Deluxe | WISO Steuer | SteuerSparErkl. | tax (Buhl) | Smartsteuer | ELSTER Online |
|---|---|---|---|---|---|---|
| **Mantelbogen (ESt 1 A)** | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage N** (Einkuenfte nichtselbst. Arbeit) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage N-AUS** (ausland. Arbeitslohn) | Yes (Deluxe) | Yes | Yes | Yes | Yes | Yes |
| **Anlage N-GRE** (Grenzpendler) | Yes (Deluxe) | Yes | Yes | Yes | Yes | Yes |
| **Anlage N-DHH** (doppelte Haushaltsfuhrung) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage V** (Vermietung/Verpachtung) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage V-FeWo** (Ferienwohnung) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage KAP** (Kapitalertrage) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage KAP-BET** (KAP Beteiligung) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage KAP-INV** (KAP Investmentertr.) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage R** (Renten inlandisch) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage R-AUS** (Renten auslandisch) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage R-AV/bAV** (Riester/betr. AV) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage SO** (sonstige Einkuenfte) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage G** (Gewerbebetrieb) | Yes | Yes | Yes | Yes (Prof.) | Yes | Yes |
| **Anlage S** (selbstandige Arbeit) | Yes | Yes | Yes | Yes (Prof.) | Yes | Yes |
| **Anlage L** (Land-/Forstwirtschaft) | Yes | Yes | Yes | Yes | Limited | Yes |
| **Anlage 13a** (LuF Durchschnittssatze) | No | Yes | Yes | Yes | No | Yes |
| **Anlage AV** (Altersvorsorge/Riester) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage Kind** (Kinder) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage K** (Kinderfreibetrag-Ubertragung) | Yes (self-fill) | Yes | Yes | Yes | Yes | Yes |
| **Anlage U** (Unterhalt) | Yes (Deluxe, self-fill) | Yes | Yes | Yes | Yes | Yes |
| **Anlage Vorsorgeaufwand** | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage FW** (Forderung Wohneigentum) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage AUS** (ausland. Einkuenfte) | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage Sonderausgaben** | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage Aussergew. Belastungen** | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage Energetische Massnahmen (35c)** | Yes | Yes | Yes | Yes | Yes | Yes |
| **Anlage 34a** (nicht entnommene Gewinne) | No | Yes | Yes | Yes | Yes | Yes |
| **Anlage Zinsschranke** | No | Yes | Yes | Yes | Yes | Yes |
| **WA-ESt** (weitere Angaben) | No | Yes | Yes | Yes | Yes | Yes |
| **Antrag auf Nichtveranlagung (NV)** | Yes (Deluxe) | Yes | Yes | Yes | No | Yes |

### 3.2 Business & Other Tax Forms

| Form | QuickSteuer Deluxe | WISO Steuer | SteuerSparErkl. | tax (Buhl) | Smartsteuer | ELSTER Online |
|---|---|---|---|---|---|---|
| **EUR (Einnahmen-Uberschuss-Rechnung)** | Yes (Deluxe) | Yes | Yes (Selbst. ed.) | Yes (Prof.) | Yes | Yes |
| **AVEUR** (Anlageverzeichnis zur EUR) | Yes | Yes | Yes | Yes (Prof.) | Yes | Yes |
| **Anlage SE** (Sonderbetriebsausgaben) | No | Yes | Yes | Yes (Prof.) | Yes | Yes |
| **Anlage ER** (Erganzungsrechnung) | No | Yes | Yes | Yes (Prof.) | No | Yes |
| **ESt 1 B** (gesonderte Feststellung) | No | Yes | Yes | Yes (Prof.) | Yes | Yes |
| **ESt 1 D** (einheitl. Feststellung) | Yes (Deluxe) | Yes | Yes | Yes (Prof.) | No | Yes |
| **FB** (Feststellungsbeteiligte) | Yes (Deluxe) | Yes | Yes | Yes (Prof.) | No | Yes |
| **FE 1-5** (Feststellungs-Anlagen) | Yes (Deluxe: FE-KAP, FE-1, FE-2) | Yes (all) | Yes | Yes (Prof.) | No | Yes |
| **Lohnsteuer-Ermassigung** | Yes | Yes (2026) | Yes | Yes | No | Yes |
| **USt-Erklarung** (Umsatzsteuer annual) | No | Yes (2025) | Yes (Selbst. ed.) | Yes (Prof.) | Yes | Yes |
| **USt-Voranmeldung** (UStVA) | Yes (Deluxe, 2026) | Yes (2026) | Yes (Selbst. ed.) | Yes (Prof.) | Yes | Yes |
| **GewSt 1 A** (Gewerbesteuer) | No | Yes (2025) | Yes (Selbst. ed.) | Yes (Prof.) | Yes | Yes |
| **GewSt 1 D** (GewSt-Zerlegung) | No | Yes | No | Yes (Prof.) | No | Yes |
| **KSt 1** (Korperschaftsteuer) | No | No | No | No | No | Yes |
| **Lohnsteuer-Anmeldung (LStA)** | No | Yes (2026) | No | No | No | Yes |
| **Grundsteuer** | No | No | No | No | No | Yes (state-specific) |
| **Erbschaft-/Schenkungsteuer** | No | No | No | No | No | Yes |
| **Kapitalertragsteuer-Anmeldung** | No | No | No | No | No | Yes |
| **Fragebogen steuerl. Erfassung** | No | No | No | No | No | Yes |

---

## 4. Supported Tax Types (Steuerarten) Matrix

| Tax Type | QuickSteuer Deluxe | WISO Steuer | SteuerSparErkl. | tax (Buhl) | Smartsteuer | ELSTER Online |
|---|---|---|---|---|---|---|
| **Einkommensteuer (ESt)** | Yes | Yes | Yes | Yes | Yes | Yes |
| **Solidaritatszuschlag** | Yes | Yes | Yes | Yes | Yes | Yes |
| **Kirchensteuer** | Yes | Yes | Yes | Yes | Yes | Yes |
| **Umsatzsteuer (USt)** | Deluxe (UStVA only) | Yes (full) | Selbst. edition | Prof. edition | Yes | Yes |
| **Gewerbesteuer (GewSt)** | No | Yes | Selbst. edition | Prof. edition | Yes | Yes |
| **Lohnsteuer** | Yes (Ermassigung) | Yes (Ermassigung + Anmeldung) | Yes (Ermassigung) | Yes | No | Yes |
| **Korperschaftsteuer (KSt)** | No | No | No | No | No | Yes |
| **Grundsteuer** | No | No | No | No | No | Yes |
| **Erbschaft-/Schenkungsteuer** | No | No | No | No | No | Yes |
| **Kapitalertragsteuer** | No | No | No | No | No | Yes |
| **Sportwetten-/Lotteriesteuer** | No | No | No | No | No | Yes |

---

## 5. ELSTER/ERiC Technical Integration

### 5.1 ELSTER Architecture Overview

ELSTER (ELektronische STeuerERklarung) is Germany's electronic tax filing system operated by the Bayerisches Landesamt fur Steuern (Bavarian State Tax Office) on behalf of all German federal states.

**Architecture layers:**
1. **Mein ELSTER** - Browser-based portal for end users (free, direct filing)
2. **ERiC (ELSTER Rich Client)** - C library for third-party software integration
3. **ELSTER Transfer** - Backend clearing house infrastructure
4. **Einfach ELSTER / Einfach ELSTER Plus** - Simplified filing tools for retirees and employees

### 5.2 ERiC (ELSTER Rich Client) SDK

**Type**: Compiled C library with defined interface specification
**Cost**: Free of charge from tax authorities
**Current Version**: ERiC 43.x (as of 2025/2026)
**Previous notable versions**: 41.4.6.0, 40.1.8.0, 39.6.4.0, 38.1.6.0, 37.2, 35.2.8.0

**Core library files:**
- `libericapi.so` (Linux) / `ericapi.dll` (Windows) - Main API library
- `libericxerces.so` / `ericxerces.dll` - XML parser (Xerces-based)
- `libeSigner.so` / `eSigner.dll` - Cryptographic signing
- `plugins2/` directory - Plugin architecture for extensions
- `ericapi.h` - C header file with API declarations

**Supported platforms:**
- Windows (x86_64)
- Linux (x86_64)
- macOS (reported but less documented)

**Key API capabilities:**
- Plausibility checking of tax data (validation against rules)
- XML data processing (single and batch data)
- Encrypted transmission to tax authority clearing house
- Cryptographic authentication (certificate-based)
- PDF generation upon successful server feedback
- Support for both single-threading and multi-threading APIs

**Key API functions (from documentation references):**
- `EricBearbeiteVorgang` - Process a tax filing operation
- `EricSende` - Send data to ELSTER servers
- Initialization and cleanup functions
- Certificate handling functions
- Error handling and status retrieval

### 5.3 Developer Registration Process

1. Register as manufacturer/developer on ELSTER developer portal
2. Undergo examination by the ICT department of the Bavarian State Tax Office
3. Receive access credentials via email
4. Download ERiC SDK using provided credentials
5. Apply for a **Hersteller-ID** (manufacturer ID) - required for production use
6. Subscribe to ELSTER newsletter for update notifications
7. Access the manufacturer forum (restricted to registered developers)

**Developer portal**: https://www.elster.de/elsterweb/infoseite/entwickler

### 5.4 XML Data Formats

**Data exchange format**: XML
- Tax data is assembled into XML documents following official ELSTER XML schemas
- XML schemas are downloadable from the developer portal
- Schemas are organized by tax type (Steuerart/Datenart)
- Documentation package: `ERiC-*-Dokumentation.zip` contains schema files in `Dokumentation/Schnittstellenbeschreibungen/`

**XML processing flow:**
1. Third-party software creates XML document according to ELSTER schema
2. ERiC validates the XML against schemas and plausibility rules
3. ERiC signs the XML using ELSTER certificate (PKCS#12 / .pfx format)
4. ERiC encrypts and transmits to ELSTER clearing house
5. Clearing house returns confirmation/error response
6. ERiC can generate PDF of the submitted declaration

**Authentication:**
- ELSTER certificates in PKCS#12 format (.pfx files)
- Certificate path and password must be provided to ERiC
- Certificates are issued through ELSTER registration process

### 5.5 ERiC-Supported Tax Data Types (Datenarten)

Based on ELSTER documentation, ERiC supports electronic transmission of:

| Category | Datenarten |
|---|---|
| **Income Tax** | ESt (Einkommensteuer), including all Anlagen |
| **Corporate Tax** | KSt (Korperschaftsteuer) |
| **Trade Tax** | GewSt, GewSt-Zerlegung |
| **Sales Tax/VAT** | UStVA (Voranmeldung), USt (Jahreserklarung), Zusammenfassende Meldung, Dauerfristverlangerung |
| **Payroll Tax** | LStA (Lohnsteuer-Anmeldung), LStB (Lohnsteuerbescheinigung), ELStAM |
| **Property Tax** | Grundsteuer (state-specific forms) |
| **Inheritance/Gift Tax** | Erbschaftsteuer, Schenkungsteuer |
| **E-Bilanz** | Electronic balance sheet (XBRL/XML) |
| **Capital Gains Tax** | Kapitalertragsteuer-Anmeldung |
| **Determination** | Gesonderte/einheitliche Feststellung |
| **EUR** | Einnahmen-Uberschuss-Rechnung |
| **OSS** | One-Stop-Shop for EU VAT |
| **Registration** | Fragebogen zur steuerlichen Erfassung |
| **Gambling taxes** | Sportwetten, Lotterie, Online-Poker, etc. |

### 5.6 Third-Party Integration Examples

**Existing open-source wrappers:**
- **Erica** (Python/FastAPI, by DigitalService/German government) - REST API wrapper around ERiC; archived July 2025. Used Python 3.11, PostgreSQL, FastAPI with Swagger docs. Libraries: pyeric wrapper.
- **eric-rs** (Rust) - Rust bindings for ERiC. Provides low-level FFI bindings and higher-level SDK. Active development.
- **JuryOberst/Elster** (.NET) - ELSTER classes and XML schema implementations
- **taxbird** (archived, Gtk2/GNU Guile) - Historical UStVA ELSTER client

**Enterprise integrations:**
- SAP PI/PO ELSTER adapter modules
- Microsoft Dynamics 365 ELSTER integration
- Oracle NetSuite German tax return submission

### 5.7 Release Cycle

- ERiC receives regular updates (approximately biannual: spring and fall releases)
- Tax form schemas are updated annually corresponding to tax year changes
- Bereitstellungstermine (availability dates) published on ELSTER portal
- Only the latest ERiC version receives official support
- Developers must update annually to maintain compatibility

---

## 6. German Tax Law Fundamentals for Software

### 6.1 Key Tax Laws (Steuergesetze)

| Law | Abbreviation | Relevance for Software |
|---|---|---|
| **Einkommensteuergesetz** | EStG | Core income tax calculation: tax brackets, deductions, all Einkunftsarten (7 types of income per Par. 2 EStG) |
| **Abgabenordnung** | AO | General tax procedures: deadlines, penalties, electronic filing requirements (Par. 87a AO), tax secrecy (Par. 30 AO) |
| **Umsatzsteuergesetz** | UStG | VAT calculation: rates (19%/7%), Kleinunternehmerregelung (Par. 19 UStG), Vorsteuerabzug, reverse charge |
| **Gewerbesteuergesetz** | GewStG | Trade tax: Hinzurechnungen, Kurzungen, Messbetrag, Hebesatz calculation |
| **Korperschaftsteuergesetz** | KStG | Corporate income tax: flat 15% rate + SolZ |
| **Solidaritatszuschlaggesetz** | SolZG | Solidarity surcharge: 5.5% of ESt/KSt (with Freigrenze since 2021) |

### 6.2 Seven Types of Income (Par. 2 EStG)

Software must handle all seven income types defined in EStG:

1. **Einkuenfte aus Land- und Forstwirtschaft** (Par. 13 EStG) - Anlage L
2. **Einkuenfte aus Gewerbebetrieb** (Par. 15 EStG) - Anlage G
3. **Einkuenfte aus selbstandiger Arbeit** (Par. 18 EStG) - Anlage S
4. **Einkuenfte aus nichtselbstandiger Arbeit** (Par. 19 EStG) - Anlage N
5. **Einkuenfte aus Kapitalvermogen** (Par. 20 EStG) - Anlage KAP
6. **Einkuenfte aus Vermietung und Verpachtung** (Par. 21 EStG) - Anlage V
7. **Sonstige Einkuenfte** (Par. 22 EStG) - Anlage SO, Anlage R

### 6.3 Tax Calculation Framework

**Income Tax (ESt) calculation chain:**
1. Sum of income from all 7 types (Summe der Einkuenfte)
2. Subtract Altersentlastungsbetrag, Entlastungsbetrag fur Alleinerziehende
3. = Gesamtbetrag der Einkuenfte
4. Subtract Verlustabzug (Par. 10d EStG)
5. Subtract Sonderausgaben (Par. 10-10c EStG)
6. Subtract Aussergewohnliche Belastungen (Par. 33-33b EStG)
7. = Einkommen
8. Subtract Kinderfreibetrage (if more favorable than Kindergeld)
9. = Zu versteuerndes Einkommen (zvE)
10. Apply tax tariff (Par. 32a EStG)
11. Add Solidaritatszuschlag + Kirchensteuer

**2025 Tax Brackets (for software version 2026):**
- 0 - 12,084 EUR: 0% (Grundfreibetrag)
- 12,085 - 68,430 EUR: 14% - 42% (progressive zone)
- 68,431 - 277,825 EUR: 42% (Proportionalzone)
- 277,826+ EUR: 45% (Reichensteuer)

**2026 Tax Brackets (projected, for future versions):**
- 0 - 12,348 EUR: 0% (Grundfreibetrag)
- 12,349 - 69,878 EUR: 14% - 42% (progressive zone)
- 69,879 - 277,825 EUR: 42%
- 277,826+ EUR: 45%

### 6.4 VAT (UStG) Key Rules for Software

- Standard rate: 19% (Par. 12 Abs. 1 UStG)
- Reduced rate: 7% (Par. 12 Abs. 2 UStG)
- Kleinunternehmerregelung (Par. 19 UStG): Exemption if prior-year revenue <= threshold (reformed 2025, now net amounts)
- Vorsteuerabzug (Par. 15 UStG): Input VAT deduction
- UStVA filing: Monthly or quarterly depending on prior-year tax liability
- Reverse charge mechanism (Par. 13b UStG)

### 6.5 Trade Tax (GewStG) Key Rules

- Applies to Gewerbebetrieb only (not Freiberufler under Par. 18 EStG)
- Calculation: Gewerbeertrag x Steuermesszahl (3.5%) x Hebesatz (municipality-specific, min. 200%)
- Hinzurechnungen (Par. 8 GewStG): Add back certain deductions
- Kuerzungen (Par. 9 GewStG): Reduce for certain items
- Freibetrag: 24,500 EUR for natural persons and partnerships
- Anrechnung on ESt (Par. 35 EStG): Factor 4.0 x Messbetrag

### 6.6 EUeR (Einnahmen-Uberschuss-Rechnung) Requirements

- Simplified profit determination per Par. 4 Abs. 3 EStG
- Cash-basis accounting (Zufluss-/Abflussprinzip, Par. 11 EStG)
- Required for Freiberufler and small businesses below thresholds
- Mandatory double-entry bookkeeping (Bilanzierung) if:
  - Revenue > 800,000 EUR, or
  - Profit > 80,000 EUR
- Must be filed electronically (Anlage EUR) via ELSTER
- Includes Anlageverzeichnis (AVEUR) for fixed assets

### 6.7 Electronic Filing Requirements

- Par. 87a AO: Legal basis for electronic communication with tax authorities
- Par. 25 Abs. 4 EStG: ESt returns may be required electronically
- Par. 18 Abs. 1 UStG: UStVA must be filed electronically
- Par. 14c Abs. 1 EStG: EUR must be filed electronically
- Hardship exemption (Harte-Fall) available per Par. 150 Abs. 8 AO

---

## 7. Differentiating Features Summary

### WISO Steuer 2026 - Key Differentiators
- **SteuerGPT**: AI-powered chatbot for tax questions and optimization
- **Broadest platform support**: Windows, Mac, iOS, Android, Browser
- **Most comprehensive**: All Einkunftsarten, EUeR, USt, GewSt, LStA in one product
- **Steuerbescheid-Prufung** with detailed discrepancy explanation
- **Einspruch** (objection) filing support
- **Multiple calculators**: Kfz-Steuer, Grundsteuer, Abfindungen, Riester
- **Testsieger** Stiftung Warentest (1.3, sehr gut)
- **Best price-to-feature ratio** at ~22.90 EUR

### Lexware QuickSteuer Deluxe 2026 - Key Differentiators
- **ELSTER-Assistent**: Step-by-step transmission wizard with real-time status
- **Steuerprufer**: Toggle-able tax checker during data entry
- **Deluxe extras**: EUeR calculator with UStVA, Feststellungserklarung
- **Self-fill forms**: Manual forms for Anlage U, K, NV, N-GRE/N-AUS
- **Part of Lexware ecosystem** (integration potential)

### SteuerSparErklarung 2026 - Key Differentiators
- **AI assistant "Alma"**: KI-powered guidance
- **CoinTracking integration**: Crypto tax import
- **Photovoltaik module**: Specific PV system tax handling (Selbststandige ed.)
- **Flex variant**: Filing without own ELSTER certificate (digital shipping service)
- **Steuerklassen-Empfehlung**: Tax class recommendation
- **Strong editorial content**: Access to Germany's largest private tax database (Plus)

### tax 2026 (Buhl) - Key Differentiators
- **Budget option**: Cheapest desktop software for basic cases
- **Professional tier**: Up to 15 declarations, full business tax support
- **Same engine as WISO**: Buhl Data Service technology base
- **Fixed asset accounting** (Anlagenbuchhaltung) with chart of accounts in EUeR
- **Berater edition**: Available for tax advisors (separate product)

### Smartsteuer - Key Differentiators
- **Fully browser-based**: No installation, any OS
- **TUV Rheinland certified**: Regular security audits
- **Free refund preview**: Calculate before paying
- **Lexware Office integration**: Direct EUeR data import from accounting software
- **smartCheck**: Automated optimization review before submission
- **No ELSTER certificate needed** for some operations

### ELSTER Online (Mein ELSTER) - Key Differentiators
- **Free**: No cost whatsoever
- **Official**: Direct government portal, always up-to-date
- **Most comprehensive form coverage**: ALL tax types including KSt, Grundsteuer, Erbschaftsteuer, gambling taxes
- **Anlage-Assistent**: Form selection wizard
- **Einfach ELSTER**: Simplified tool for retirees
- **Einfach ELSTER Plus**: Simplified tool for employees
- **MeinELSTER+ App**: Mobile filing (NRW pilot from July 2026)
- **Receipt storage**: Digital Belegablage in user account
- **No guidance/optimization**: Raw form filling, no tax tips or optimization
- **No Steuerbescheid-Prufung**: No comparison with own calculation

---

## 8. Sources

- [Lexware QuickSteuer Shop](https://shop.lexware.de/quicksteuer)
- [QuickSteuer Funktionen Vergleich](https://www.quicksteuer.de/funktionen)
- [QuickSteuer Deluxe 2026 - Markt+Technik](https://mut.de/products/lexware-quicksteuer-2026-deluxe-fur-das-steuerjahr-2025)
- [WISO Steuer 2026 - Ashampoo](https://www.ashampoo.com/de-de/wiso-steuer-2026)
- [WISO Steuer Testsieger](https://www.buhl.de/steuer/test-ergebnisse/)
- [SteuerSparErklarung - Akademische Arbeitsgemeinschaft](https://www.akademische.de/presse/produktinformationen)
- [SteuerSparErklarung 2026 - Ashampoo](https://www.ashampoo.com/de-de/steuersparerklaerung-2026)
- [SteuerSparErklarung Selbststandige - steuertipps.de](https://www.steuertipps.de/shop/software/steuer-spar-erklaerung-selbststaendige)
- [Buhl tax 2026 - Ashampoo](https://www.ashampoo.com/de-de/buhl-tax-2026)
- [Buhl tax 2026 Professional - Lizenzguru](https://lizenzguru.de/buhl-tax-2026-professional-fuer-das-steuerjahr-2025.html)
- [Smartsteuer Funktionen und Features](https://www.smartsteuer.de/online/smartsteuer-funktionen-und-features/)
- [Smartsteuer Funktionsumfang 2023](https://www.smartsteuer.de/online/funktionsumfang-2023/)
- [ELSTER Alle Formulare](https://www.elster.de/eportal/formulare-leistungen/alleformulare)
- [ELSTER Developer Portal](https://www.elster.de/elsterweb/infoseite/entwickler?locale=en_US)
- [ELSTER Bereitstellungstermine](https://www.elster.de/eportal/infoseite/bereitstellungstermine)
- [Erica (ERiC wrapper) - GitHub](https://github.com/digitalservicebund/erica)
- [eric-rs (Rust ERiC bindings) - GitHub](https://github.com/quambene/eric-rs)
- [ELSTER Schnittstelle - Recht Logisch](https://rechtlogisch.de/elster-schnittstelle/)
- [Steuersoftware Vergleich 2026](https://www.steuersoftware-tests.de/steuersoftware-vergleich.html)
- [Stiftung Warentest Steuerprogramme](https://www.test.de/Steuerprogramme-im-Test-5165521-0/)
- [German Fiscal Code (AO) in English](https://www.gesetze-im-internet.de/englisch_ao/)
- [365 Business ERiC ELSTER Steuerdatenarten](https://docs.365businessdev.com/en-us/365-business-eric/elster-tax-statements.html)
