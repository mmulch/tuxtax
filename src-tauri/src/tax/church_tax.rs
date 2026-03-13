/// Kirchensteuer (8% in BaWü/Bayern, 9% in übrigen Bundesländern)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChurchTaxRate {
    /// Bayern, Baden-Württemberg: 8%
    Eight,
    /// Alle anderen Bundesländer: 9%
    Nine,
    /// Kein Kirchensteuerpflichtiger
    None,
}

impl ChurchTaxRate {
    pub fn rate(&self) -> f64 {
        match self {
            ChurchTaxRate::Eight => 0.08,
            ChurchTaxRate::Nine => 0.09,
            ChurchTaxRate::None => 0.0,
        }
    }

    pub fn from_bundesland(bundesland: &str) -> Self {
        match bundesland {
            "BW" | "BY" => ChurchTaxRate::Eight,
            "" => ChurchTaxRate::None,
            _ => ChurchTaxRate::Nine,
        }
    }
}

pub fn calculate_church_tax(income_tax: f64, rate: &ChurchTaxRate) -> f64 {
    (income_tax * rate.rate()).floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_church_tax() {
        assert_eq!(calculate_church_tax(10000.0, &ChurchTaxRate::None), 0.0);
    }

    #[test]
    fn test_bayern_rate() {
        assert_eq!(calculate_church_tax(10000.0, &ChurchTaxRate::Eight), 800.0);
    }

    #[test]
    fn test_nrw_rate() {
        assert_eq!(calculate_church_tax(10000.0, &ChurchTaxRate::Nine), 900.0);
    }
}
