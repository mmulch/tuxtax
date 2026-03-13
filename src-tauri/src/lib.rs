mod commands;
mod db;
mod elster;
pub mod error;
mod forms;
mod interview;
mod services;
mod tax;

use db::Database;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

/// Application state shared across Tauri commands
pub struct AppState {
    pub db: Database,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging (configurable via RUST_LOG env var)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("tuxtax=info")),
        )
        .init();

    tracing::info!("TuxTax v{} starting", env!("CARGO_PKG_VERSION"));

    // Open database (unencrypted for now, encryption added when user sets passphrase)
    let data_dir = db::data_dir().expect("could not determine data directory");
    let db_path = data_dir.join("tuxtax.db");
    tracing::info!("Database path: {}", db_path.display());

    let database =
        Database::open_unencrypted(&db_path).expect("could not open database");

    let state = Arc::new(AppState { db: database });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::tax_case::create_tax_case,
            commands::tax_case::list_tax_cases,
            commands::tax_case::get_tax_case,
            commands::tax_case::delete_tax_case,
            commands::calculation::calculate_tax,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
