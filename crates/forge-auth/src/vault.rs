use std::collections::HashMap;
use std::path::Path;

use forge_core::error::{ForgeError, ForgeResult};
use rusqlite::Connection;

pub struct SecretVault {
    conn: Connection,
}

impl SecretVault {
    pub fn open(db_path: &Path) -> ForgeResult<Self> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS secrets (
                name TEXT PRIMARY KEY,
                encrypted_value BLOB NOT NULL,
                scope TEXT NOT NULL DEFAULT 'project',
                created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
                updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
            )",
        )?;
        Ok(Self { conn })
    }

    pub fn in_memory() -> ForgeResult<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS secrets (
                name TEXT PRIMARY KEY,
                encrypted_value BLOB NOT NULL,
                scope TEXT NOT NULL DEFAULT 'project',
                created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
                updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
            )",
        )?;
        Ok(Self { conn })
    }

    pub fn set_secret(&self, name: &str, value: &[u8], scope: &str) -> ForgeResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO secrets (name, encrypted_value, scope,
             updated_at) VALUES (?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
            rusqlite::params![name, value, scope],
        )?;
        Ok(())
    }

    pub fn get_secret(&self, name: &str) -> ForgeResult<Vec<u8>> {
        self.conn
            .prepare("SELECT encrypted_value FROM secrets WHERE name = ?1")?
            .query_row([name], |row| row.get(0))
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => ForgeError::NotFound {
                    entity: "secret".to_string(),
                    id: name.to_string(),
                },
                other => ForgeError::Database(other.to_string()),
            })
    }

    pub fn delete_secret(&self, name: &str) -> ForgeResult<()> {
        self.conn
            .execute("DELETE FROM secrets WHERE name = ?1", [name])?;
        Ok(())
    }

    pub fn list_secrets(&self) -> ForgeResult<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM secrets ORDER BY name")?;
        let names = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(names)
    }
}
