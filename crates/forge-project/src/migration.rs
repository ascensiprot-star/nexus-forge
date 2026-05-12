use forge_core::error::ForgeResult;
use rusqlite::Connection;

const MIGRATIONS: &[(&str, &str)] = &[
    (
        "001_initial",
        include_str!("../../../schemas/migrations/001_initial.sql"),
    ),
    (
        "002_memory_layers",
        include_str!("../../../schemas/migrations/002_memory_layers.sql"),
    ),
    (
        "003_audit_trail",
        include_str!("../../../schemas/migrations/003_audit_trail.sql"),
    ),
];

pub fn run_migrations(conn: &Connection) -> ForgeResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
        )",
    )?;

    for (version, sql) in MIGRATIONS {
        let applied: bool = conn
            .prepare("SELECT COUNT(*) FROM schema_migrations WHERE version = ?1")?
            .query_row([version], |row| row.get::<_, i64>(0))
            .map(|count| count > 0)?;

        if !applied {
            tracing::info!("applying migration: {version}");
            conn.execute_batch(sql)?;
            conn.execute(
                "INSERT INTO schema_migrations (version) VALUES (?1)",
                [version],
            )?;
        }
    }

    Ok(())
}
