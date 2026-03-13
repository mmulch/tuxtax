pub mod documents;
pub mod migrations;
pub mod schema;
pub mod tax_case;

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::error::AppError;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Opens or creates a database at the given path with SQLCipher encryption.
    pub fn open(path: &PathBuf, passphrase: &str) -> Result<Self, AppError> {
        let conn = Connection::open(path)?;

        // Set SQLCipher encryption key
        conn.pragma_update(None, "key", passphrase)?;

        // Verify the database is accessible (will fail if wrong key)
        conn.pragma_query_value(None, "cipher_version", |row| row.get::<_, String>(0))
            .map_err(|_| AppError::Database("Failed to open database: invalid passphrase or corrupted file".into()))?;

        // Enable WAL mode for better concurrent read performance
        conn.pragma_update(None, "journal_mode", "WAL")?;

        // Run migrations
        migrations::run(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Opens an unencrypted database (for development/testing).
    pub fn open_unencrypted(path: &PathBuf) -> Result<Self, AppError> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        migrations::run(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Opens an in-memory database (for testing).
    #[cfg(test)]
    pub fn open_in_memory() -> Result<Self, AppError> {
        let conn = Connection::open_in_memory()?;
        migrations::run(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().expect("database mutex poisoned")
    }
}

/// Returns the default data directory for TuxTax.
pub fn data_dir() -> Result<PathBuf, AppError> {
    let dir = dirs::data_dir()
        .ok_or_else(|| AppError::Config("Could not determine data directory".into()))?
        .join("tuxtax");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
