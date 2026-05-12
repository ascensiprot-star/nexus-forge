use chrono::{DateTime, Utc};
use forge_core::error::ForgeResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub sequence_number: u64,
    pub previous_hash: String,
    pub entry_hash: String,
    pub timestamp: DateTime<Utc>,
    pub actor_type: String,
    pub actor_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub outcome: String,
    pub details: serde_json::Value,
    pub correlation_id: Option<String>,
}

pub struct AuditLog {
    entries: Vec<AuditEntry>,
    last_hash: String,
    next_sequence: u64,
}

impl AuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            last_hash: "genesis".to_string(),
            next_sequence: 1,
        }
    }

    pub fn append(
        &mut self,
        actor_type: &str,
        actor_id: &str,
        action: &str,
        resource_type: &str,
        resource_id: &str,
        outcome: &str,
        details: serde_json::Value,
    ) -> ForgeResult<String> {
        let entry_data = format!(
            "{}:{}:{}:{}:{}:{}:{}",
            self.next_sequence,
            self.last_hash,
            actor_type,
            actor_id,
            action,
            resource_type,
            resource_id
        );

        use sha2::{Digest, Sha256};
        let hash = format!("{:x}", Sha256::digest(entry_data.as_bytes()));

        let entry = AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            sequence_number: self.next_sequence,
            previous_hash: self.last_hash.clone(),
            entry_hash: hash.clone(),
            timestamp: Utc::now(),
            actor_type: actor_type.to_string(),
            actor_id: actor_id.to_string(),
            action: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: resource_id.to_string(),
            outcome: outcome.to_string(),
            details,
            correlation_id: None,
        };

        self.last_hash = hash.clone();
        self.next_sequence += 1;
        self.entries.push(entry);

        Ok(hash)
    }

    pub fn verify_chain(&self) -> bool {
        let mut expected_hash = "genesis".to_string();
        for entry in &self.entries {
            if entry.previous_hash != expected_hash {
                return false;
            }
            expected_hash = entry.entry_hash.clone();
        }
        true
    }

    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_chain() {
        let mut log = AuditLog::new();
        log.append(
            "user",
            "u1",
            "create",
            "project",
            "p1",
            "success",
            serde_json::json!({}),
        )
        .unwrap();
        log.append(
            "agent",
            "a1",
            "execute",
            "task",
            "t1",
            "success",
            serde_json::json!({}),
        )
        .unwrap();

        assert!(log.verify_chain());
        assert_eq!(log.entries().len(), 2);
    }
}
