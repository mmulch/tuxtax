use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxCaseSummary {
    pub id: i64,
    pub year: u16,
    pub name: String,
    pub status: String,
}

#[tauri::command]
pub fn create_tax_case(
    state: State<'_, Arc<AppState>>,
    year: u16,
    name: String,
) -> Result<TaxCaseSummary, String> {
    let conn = state.db.conn();
    conn.execute(
        "INSERT INTO tax_cases (year, name) VALUES (?1, ?2)",
        rusqlite::params![year, name],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    Ok(TaxCaseSummary {
        id,
        year,
        name,
        status: "draft".to_string(),
    })
}

#[tauri::command]
pub fn list_tax_cases(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<TaxCaseSummary>, String> {
    let conn = state.db.conn();
    let mut stmt = conn
        .prepare("SELECT id, year, name, status FROM tax_cases ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let cases = stmt
        .query_map([], |row| {
            Ok(TaxCaseSummary {
                id: row.get(0)?,
                year: row.get(1)?,
                name: row.get(2)?,
                status: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(cases)
}

#[tauri::command]
pub fn get_tax_case(
    state: State<'_, Arc<AppState>>,
    id: i64,
) -> Result<TaxCaseSummary, String> {
    let conn = state.db.conn();
    conn.query_row(
        "SELECT id, year, name, status FROM tax_cases WHERE id = ?1",
        [id],
        |row| {
            Ok(TaxCaseSummary {
                id: row.get(0)?,
                year: row.get(1)?,
                name: row.get(2)?,
                status: row.get(3)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_tax_case(
    state: State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), String> {
    let conn = state.db.conn();
    let affected = conn
        .execute("DELETE FROM tax_cases WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;

    if affected == 0 {
        Err(format!("Tax case {} not found", id))
    } else {
        tracing::info!("Deleted tax case {}", id);
        Ok(())
    }
}
