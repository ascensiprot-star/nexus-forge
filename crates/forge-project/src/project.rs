use std::path::Path;
use std::sync::Arc;

use forge_core::error::{ForgeError, ForgeResult};
use forge_core::types::{Project, ProjectId, ProjectStatus};
use rusqlite::Connection;
use tokio::sync::Mutex;

use crate::migration;

pub struct ProjectStore {
    conn: Arc<Mutex<Connection>>,
}

impl ProjectStore {
    pub fn open(db_path: &Path) -> ForgeResult<Self> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        migration::run_migrations(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn in_memory() -> ForgeResult<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        migration::run_migrations(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn create_project(&self, name: &str, root_path: &str) -> ForgeResult<Project> {
        let id = ProjectId::new();
        let conn = self.conn.lock().await;

        conn.execute(
            "INSERT INTO projects (id, name, root_path, status) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id.as_str(), name, root_path, "empty"],
        )?;

        self.get_project_inner(&conn, &id)
    }

    pub async fn get_project(&self, id: &ProjectId) -> ForgeResult<Project> {
        let conn = self.conn.lock().await;
        self.get_project_inner(&conn, id)
    }

    fn get_project_inner(&self, conn: &Connection, id: &ProjectId) -> ForgeResult<Project> {
        conn.prepare(
            "SELECT id, name, root_path, status, settings, created_at, updated_at
             FROM projects WHERE id = ?1",
        )?
        .query_row([id.as_str()], |row| {
            let status_str: String = row.get(3)?;
            let settings_str: String = row.get(4)?;
            let created_str: String = row.get(5)?;
            let updated_str: String = row.get(6)?;

            Ok(Project {
                id: ProjectId::from_str(&row.get::<_, String>(0)?),
                name: row.get(1)?,
                root_path: row.get(2)?,
                status: status_str.parse().unwrap_or(ProjectStatus::Empty),
                settings: serde_json::from_str(&settings_str).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => ForgeError::NotFound {
                entity: "project".to_string(),
                id: id.to_string(),
            },
            other => ForgeError::Database(other.to_string()),
        })
    }

    pub async fn list_projects(&self) -> ForgeResult<Vec<Project>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, root_path, status, settings, created_at, updated_at
             FROM projects ORDER BY updated_at DESC",
        )?;

        let projects = stmt
            .query_map([], |row| {
                let status_str: String = row.get(3)?;
                let settings_str: String = row.get(4)?;
                let created_str: String = row.get(5)?;
                let updated_str: String = row.get(6)?;

                Ok(Project {
                    id: ProjectId::from_str(&row.get::<_, String>(0)?),
                    name: row.get(1)?,
                    root_path: row.get(2)?,
                    status: status_str.parse().unwrap_or(ProjectStatus::Empty),
                    settings: serde_json::from_str(&settings_str).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    pub async fn update_status(&self, id: &ProjectId, status: ProjectStatus) -> ForgeResult<()> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(
            "UPDATE projects SET status = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id = ?2",
            rusqlite::params![status.to_string(), id.as_str()],
        )?;

        if rows == 0 {
            return Err(ForgeError::NotFound {
                entity: "project".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    pub async fn update_settings(
        &self,
        id: &ProjectId,
        settings: serde_json::Value,
    ) -> ForgeResult<()> {
        let conn = self.conn.lock().await;
        let settings_str = serde_json::to_string(&settings)?;
        let rows = conn.execute(
            "UPDATE projects SET settings = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id = ?2",
            rusqlite::params![settings_str, id.as_str()],
        )?;

        if rows == 0 {
            return Err(ForgeError::NotFound {
                entity: "project".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    pub async fn delete_project(&self, id: &ProjectId) -> ForgeResult<()> {
        let conn = self.conn.lock().await;
        let rows = conn.execute("DELETE FROM projects WHERE id = ?1", [id.as_str()])?;

        if rows == 0 {
            return Err(ForgeError::NotFound {
                entity: "project".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_get_project() {
        let store = ProjectStore::in_memory().unwrap();
        let project = store
            .create_project("test-project", "/tmp/test")
            .await
            .unwrap();

        assert_eq!(project.name, "test-project");
        assert_eq!(project.root_path, "/tmp/test");
        assert_eq!(project.status, ProjectStatus::Empty);

        let fetched = store.get_project(&project.id).await.unwrap();
        assert_eq!(fetched.name, "test-project");
    }

    #[tokio::test]
    async fn test_list_projects() {
        let store = ProjectStore::in_memory().unwrap();
        store.create_project("project-1", "/tmp/1").await.unwrap();
        store.create_project("project-2", "/tmp/2").await.unwrap();

        let projects = store.list_projects().await.unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[tokio::test]
    async fn test_update_status() {
        let store = ProjectStore::in_memory().unwrap();
        let project = store.create_project("test", "/tmp/test").await.unwrap();

        store
            .update_status(&project.id, ProjectStatus::Planning)
            .await
            .unwrap();

        let fetched = store.get_project(&project.id).await.unwrap();
        assert_eq!(fetched.status, ProjectStatus::Planning);
    }

    #[tokio::test]
    async fn test_delete_project() {
        let store = ProjectStore::in_memory().unwrap();
        let project = store.create_project("test", "/tmp/test").await.unwrap();

        store.delete_project(&project.id).await.unwrap();

        let result = store.get_project(&project.id).await;
        assert!(result.is_err());
    }
}
