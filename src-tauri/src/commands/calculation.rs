use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TaxResult {
    pub taxable_income: f64,
    pub income_tax: f64,
    pub solidarity_surcharge: f64,
    pub church_tax: f64,
    pub total_tax: f64,
    pub refund_or_payment: f64,
}

#[tauri::command]
pub fn calculate_tax(tax_case_id: i64) -> Result<TaxResult, String> {
    // TODO: load tax case data from DB and run calculation engine
    Err(format!(
        "Calculation not yet implemented for tax case {}",
        tax_case_id
    ))
}
