use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use forge_core::error::{ForgeError, ForgeResult};
use forge_core::traits::MemoryService;
use forge_core::types::*;
use rusqlite::Connection;
use tokio::sync::Mutex;

pub struct MemoryStore {
    conn: Arc<Mutex<Connection>>,
}

impl MemoryStore {
    pub fn open(db_path: &Path) -> ForgeResult<Self> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn in_memory() -> ForgeResult<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(include_str!(
            "../../../schemas/migrations/002_memory_layers.sql"
        ))?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[async_trait]
impl MemoryService for MemoryStore {
    async fn store(&self, item: MemoryItem) -> ForgeResult<MemoryItemId> {
        let conn = self.conn.lock().await;
        let tags_json = serde_json::to_string(&item.tags)?;
        let metadata_json = serde_json::to_string(&item.metadata)?;

        conn.execute(
            "INSERT OR REPLACE INTO memory_items
             (id, project_id, layer, item_type, key, title, content, source,
              confidence, tags, metadata, embedding_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![
                item.id.as_str(),
                item.project_id.as_str(),
                item.layer.to_string(),
                item.item_type,
                item.key,
                item.title,
                item.content,
                item.source,
                item.confidence,
                tags_json,
                metadata_json,
                Option::<String>::None,
            ],
        )?;

        Ok(item.id)
    }

    async fn retrieve(
        &self,
        project_id: &ProjectId,
        layer: MemoryLayer,
        key: &str,
    ) -> ForgeResult<Option<MemoryItem>> {
        let conn = self.conn.lock().await;

        conn.execute(
            "UPDATE memory_items SET access_count = access_count + 1,
             last_accessed_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE project_id = ?1 AND layer = ?2 AND key = ?3",
            rusqlite::params![project_id.as_str(), layer.to_string(), key],
        )?;

        let result = conn
            .prepare(
                "SELECT id, project_id, layer, item_type, key, title, content, source,
                        confidence, tags, metadata, access_count, last_accessed_at,
                        created_at, updated_at, archived_at
                 FROM memory_items
                 WHERE project_id = ?1 AND layer = ?2 AND key = ?3 AND archived_at IS NULL",
            )?
            .query_row(
                rusqlite::params![project_id.as_str(), layer.to_string(), key],
                |row| Ok(row_to_memory_item(row)),
            );

        match result {
            Ok(item) => Ok(Some(item?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(ForgeError::Database(e.to_string())),
        }
    }

    async fn search(
        &self,
        project_id: &ProjectId,
        query: &str,
        layers: &[MemoryLayer],
        limit: usize,
    ) -> ForgeResult<Vec<MemoryItem>> {
        let conn = self.conn.lock().await;
        let layer_strs: Vec<String> = layers.iter().map(|l| l.to_string()).collect();
        let placeholders: Vec<String> = (0..layer_strs.len())
            .map(|i| format!("?{}", i + 3))
            .collect();

        let sql = format!(
            "SELECT id, project_id, layer, item_type, key, title, content, source,
                    confidence, tags, metadata, access_count, last_accessed_at,
                    created_at, updated_at, archived_at
             FROM memory_items
             WHERE project_id = ?1
               AND archived_at IS NULL
               AND (title LIKE '%' || ?2 || '%' OR content LIKE '%' || ?2 || '%')
               {}
             ORDER BY access_count DESC, updated_at DESC
             LIMIT {}",
            if !layer_strs.is_empty() {
                format!("AND layer IN ({})", placeholders.join(", "))
            } else {
                String::new()
            },
            limit
        );

        let mut stmt = conn.prepare(&sql)?;

        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![
            Box::new(project_id.as_str().to_string()),
            Box::new(query.to_string()),
        ];
        for l in &layer_strs {
            params.push(Box::new(l.clone()));
        }

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            params.iter().map(|p| p.as_ref()).collect();

        let items = stmt
            .query_map(param_refs.as_slice(), |row| Ok(row_to_memory_item(row)))?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    async fn update(&self, id: &MemoryItemId, content: &str) -> ForgeResult<()> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(
            "UPDATE memory_items SET content = ?1,
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id = ?2",
            rusqlite::params![content, id.as_str()],
        )?;

        if rows == 0 {
            return Err(ForgeError::NotFound {
                entity: "memory_item".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    async fn archive(&self, id: &MemoryItemId) -> ForgeResult<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE memory_items SET archived_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id = ?1",
            [id.as_str()],
        )?;
        Ok(())
    }

    async fn list_by_layer(
        &self,
        project_id: &ProjectId,
        layer: MemoryLayer,
        limit: usize,
        offset: usize,
    ) -> ForgeResult<Vec<MemoryItem>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, layer, item_type, key, title, content, source,
                    confidence, tags, metadata, access_count, last_accessed_at,
                    created_at, updated_at, archived_at
             FROM memory_items
             WHERE project_id = ?1 AND layer = ?2 AND archived_at IS NULL
             ORDER BY updated_at DESC
             LIMIT ?3 OFFSET ?4",
        )?;

        let items = stmt
            .query_map(
                rusqlite::params![project_id.as_str(), layer.to_string(), limit, offset],
                |row| Ok(row_to_memory_item(row)),
            )?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    async fn store_decision(&self, decision: Decision) -> ForgeResult<String> {
        let conn = self.conn.lock().await;
        let rejected_json = serde_json::to_string(&decision.rejected_options)?;
        let tradeoffs_json = serde_json::to_string(&decision.tradeoffs)?;
        let constraints_json = serde_json::to_string(&decision.constraints)?;

        conn.execute(
            "INSERT INTO decisions
             (id, project_id, decision_type, title, question, chosen_option,
              rejected_options, reasoning, tradeoffs, constraints, decided_by,
              task_id, supersedes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            rusqlite::params![
                decision.id,
                decision.project_id.as_str(),
                decision.decision_type,
                decision.title,
                decision.question,
                decision.chosen_option,
                rejected_json,
                decision.reasoning,
                tradeoffs_json,
                constraints_json,
                decision.decided_by,
                decision.task_id.as_ref().map(|t| t.as_str().to_string()),
                decision.supersedes,
            ],
        )?;

        Ok(decision.id)
    }

    async fn get_decisions(
        &self,
        project_id: &ProjectId,
        limit: usize,
    ) -> ForgeResult<Vec<Decision>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, decision_type, title, question, chosen_option,
                    rejected_options, reasoning, tradeoffs, constraints, decided_by,
                    task_id, supersedes, created_at
             FROM decisions
             WHERE project_id = ?1
             ORDER BY created_at DESC
             LIMIT ?2",
        )?;

        let decisions = stmt
            .query_map(rusqlite::params![project_id.as_str(), limit], |row| {
                let rejected_str: String = row.get(6)?;
                let tradeoffs_str: String = row.get(8)?;
                let constraints_str: String = row.get(9)?;
                let task_id_opt: Option<String> = row.get(11)?;
                let created_str: String = row.get(13)?;

                Ok(Decision {
                    id: row.get(0)?,
                    project_id: ProjectId::from_str(&row.get::<_, String>(1)?),
                    decision_type: row.get(2)?,
                    title: row.get(3)?,
                    question: row.get(4)?,
                    chosen_option: row.get(5)?,
                    rejected_options: serde_json::from_str(&rejected_str).unwrap_or_default(),
                    reasoning: row.get(7)?,
                    tradeoffs: serde_json::from_str(&tradeoffs_str).unwrap_or_default(),
                    constraints: serde_json::from_str(&constraints_str).unwrap_or_default(),
                    decided_by: row.get(10)?,
                    task_id: task_id_opt.map(|s| TaskId::from_str(&s)),
                    supersedes: row.get(12)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(decisions)
    }
}

fn row_to_memory_item(row: &rusqlite::Row) -> ForgeResult<MemoryItem> {
    let tags_str: String = row
        .get(9)
        .map_err(|e| ForgeError::Database(e.to_string()))?;
    let metadata_str: String = row
        .get(10)
        .map_err(|e| ForgeError::Database(e.to_string()))?;
    let created_str: String = row
        .get(13)
        .map_err(|e| ForgeError::Database(e.to_string()))?;
    let updated_str: String = row
        .get(14)
        .map_err(|e| ForgeError::Database(e.to_string()))?;
    let layer_str: String = row
        .get(2)
        .map_err(|e| ForgeError::Database(e.to_string()))?;

    Ok(MemoryItem {
        id: MemoryItemId::from_str(
            &row.get::<_, String>(0)
                .map_err(|e| ForgeError::Database(e.to_string()))?,
        ),
        project_id: ProjectId::from_str(
            &row.get::<_, String>(1)
                .map_err(|e| ForgeError::Database(e.to_string()))?,
        ),
        layer: layer_str
            .parse()
            .map_err(|e: String| ForgeError::Database(e))?,
        item_type: row
            .get(3)
            .map_err(|e| ForgeError::Database(e.to_string()))?,
        key: row
            .get(4)
            .map_err(|e| ForgeError::Database(e.to_string()))?,
        title: row
            .get(5)
            .map_err(|e| ForgeError::Database(e.to_string()))?,
        content: row
            .get(6)
            .map_err(|e| ForgeError::Database(e.to_string()))?,
        source: row
            .get(7)
            .map_err(|e| ForgeError::Database(e.to_string()))?,
        confidence: row
            .get(8)
            .map_err(|e| ForgeError::Database(e.to_string()))?,
        tags: serde_json::from_str(&tags_str).unwrap_or_default(),
        metadata: serde_json::from_str(&metadata_str).unwrap_or_default(),
        access_count: row
            .get::<_, i64>(11)
            .map_err(|e| ForgeError::Database(e.to_string()))? as u64,
        last_accessed_at: row
            .get::<_, Option<String>>(12)
            .map_err(|e| ForgeError::Database(e.to_string()))?
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
        created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        archived_at: row
            .get::<_, Option<String>>(15)
            .map_err(|e| ForgeError::Database(e.to_string()))?
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn test_item(project_id: &ProjectId) -> MemoryItem {
        MemoryItem {
            id: MemoryItemId::new(),
            project_id: project_id.clone(),
            layer: MemoryLayer::Project,
            item_type: "convention".to_string(),
            key: "naming-convention".to_string(),
            title: "Naming Convention".to_string(),
            content: "Use snake_case for functions".to_string(),
            source: "user".to_string(),
            confidence: 1.0,
            tags: vec!["style".to_string()],
            metadata: serde_json::json!({}),
            access_count: 0,
            last_accessed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            archived_at: None,
        }
    }

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let store = MemoryStore::in_memory().unwrap();
        let pid = ProjectId::new();
        let item = test_item(&pid);

        store.store(item).await.unwrap();

        let retrieved = store
            .retrieve(&pid, MemoryLayer::Project, "naming-convention")
            .await
            .unwrap();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Naming Convention");
    }

    #[tokio::test]
    async fn test_search() {
        let store = MemoryStore::in_memory().unwrap();
        let pid = ProjectId::new();
        let item = test_item(&pid);
        store.store(item).await.unwrap();

        let results = store
            .search(&pid, "snake_case", &[MemoryLayer::Project], 10)
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
    }
}
