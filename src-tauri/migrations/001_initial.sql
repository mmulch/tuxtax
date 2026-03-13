-- Initial schema: tax cases and personal data

CREATE TABLE tax_cases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    year INTEGER NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    assessment_type TEXT NOT NULL DEFAULT 'single',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE persons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tax_case_id INTEGER NOT NULL REFERENCES tax_cases(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'taxpayer',
    first_name TEXT,
    last_name TEXT,
    date_of_birth TEXT,
    tax_id TEXT,
    street TEXT,
    house_number TEXT,
    postal_code TEXT,
    city TEXT,
    federal_state TEXT,
    religion TEXT,
    tax_office_id TEXT,
    tax_number TEXT,
    iban TEXT,
    bic TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_persons_tax_case ON persons(tax_case_id);
CREATE INDEX idx_tax_cases_year ON tax_cases(year);
