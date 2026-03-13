/// Solidaritätszuschlag (5,5% mit Freigrenze und Milderungszone)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidarityParams {
    pub year: u16,
    pub rate: f64,
    /// Freigrenze (ab 2021: 16.956 EUR Grundtarif / 33.912 EUR Splitting)
    pub exemption_single: f64,
    pub exemption_joint: f64,
    /// Milderungszone: Grenzbelastung in der Gleitzone
    pub mitigation_rate: f64,
}

pub fn calculate_solidarity_surcharge(
    income_tax: f64,
    is_joint: bool,
    params: &SolidarityParams,
) -> f64 {
    let exemption = if is_joint {
        params.exemption_joint
    } else {
        params.exemption_single
    };

    if income_tax <= exemption {
        return 0.0;
    }

    let full_soli = income_tax * params.rate;
    let mitigated = (income_tax - exemption) * params.mitigation_rate;

    full_soli.min(mitigated).floor() / 100.0 * 100.0 // auf volle Cent abrunden
}

#[cfg(test)]
mod tests {
    // TODO: Tests mit realen Werten
}
