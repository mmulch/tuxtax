/// ESt-Tarif nach § 32a EStG
///
/// Berechnet die Einkommensteuer für ein gegebenes zu versteuerndes Einkommen (zvE)
/// und Steuerjahr. Die Tarifzonen und Formeln werden datengetrieben pro Jahr konfiguriert.

use serde::{Deserialize, Serialize};

/// Tarifparameter für ein Steuerjahr (§ 32a EStG)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariffParams {
    pub year: u16,
    /// Grundfreibetrag (Zone 1 Obergrenze)
    pub basic_allowance: f64,
    /// Obergrenze Zone 2
    pub zone2_upper: f64,
    /// Obergrenze Zone 3
    pub zone3_upper: f64,
    /// Obergrenze Zone 4 (ab hier Spitzensteuersatz)
    pub zone4_upper: f64,
    /// Faktor y-Formel Zone 2
    pub zone2_factor_a: f64,
    pub zone2_factor_b: f64,
    /// Faktor z-Formel Zone 3
    pub zone3_factor_a: f64,
    pub zone3_factor_b: f64,
    /// Steuersatz Zone 4 (42%)
    pub zone4_rate: f64,
    /// Steuersatz Zone 5 / Reichensteuer (45%)
    pub zone5_rate: f64,
}

/// Berechnet die tarifliche Einkommensteuer (Grundtarif)
pub fn calculate_income_tax(zve: f64, params: &TariffParams) -> f64 {
    let zve = zve.floor();
    if zve <= 0.0 {
        return 0.0;
    }

    if zve <= params.basic_allowance {
        // Zone 1: keine Steuer
        0.0
    } else if zve <= params.zone2_upper {
        // Zone 2: progressiver Einstieg
        let y = (zve - params.basic_allowance) / 10_000.0;
        (params.zone2_factor_a * y + params.zone2_factor_b) * y
    } else if zve <= params.zone3_upper {
        // Zone 3: weiterer progressiver Anstieg
        let z = (zve - params.zone2_upper) / 10_000.0;
        (params.zone3_factor_a * z + params.zone3_factor_b) * z
            + params.zone3_base_tax()
    } else if zve <= params.zone4_upper {
        // Zone 4: linearer Spitzensteuersatz (42%)
        params.zone4_rate * zve - params.zone4_deduction()
    } else {
        // Zone 5: Reichensteuer (45%)
        params.zone5_rate * zve - params.zone5_deduction()
    }
    .floor()
}

impl TariffParams {
    /// Steuerbetrag am Ende von Zone 2 (Basis für Zone 3)
    fn zone3_base_tax(&self) -> f64 {
        let y = (self.zone2_upper - self.basic_allowance) / 10_000.0;
        (self.zone2_factor_a * y + self.zone2_factor_b) * y
    }

    /// Abzugsbetrag für Zone 4 (damit Übergang stetig ist)
    fn zone4_deduction(&self) -> f64 {
        // Wird beim Laden der Jahresparameter berechnet
        0.0 // TODO: aus Jahresparametern ableiten
    }

    /// Abzugsbetrag für Zone 5
    fn zone5_deduction(&self) -> f64 {
        0.0 // TODO: aus Jahresparametern ableiten
    }
}

#[cfg(test)]
mod tests {
    // TODO: Tests mit realen Steuertabellen-Werten
}
