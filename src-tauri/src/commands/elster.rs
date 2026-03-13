use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SubmissionResult {
    pub transfer_ticket: String,
    pub timestamp: String,
}

// Commands will be registered once ELSTER integration is ready
pub fn validate_tax_case(_tax_case_id: i64) -> Result<ValidationResult, String> {
    Err("ELSTER integration not yet available".to_string())
}

pub fn submit_tax_case(_tax_case_id: i64, _certificate_path: String, _pin: String) -> Result<SubmissionResult, String> {
    Err("ELSTER integration not yet available".to_string())
}
