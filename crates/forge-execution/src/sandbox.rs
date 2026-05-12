use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use forge_core::error::{ForgeError, ForgeResult};
use forge_core::traits::ExecutionService;
use forge_core::types::*;
use tokio::sync::RwLock;

use crate::process::ProcessRunner;

pub struct SandboxManager {
    sandboxes: Arc<RwLock<HashMap<SandboxId, SandboxConfig>>>,
    runner: ProcessRunner,
}

impl SandboxManager {
    pub fn new() -> Self {
        Self {
            sandboxes: Arc::new(RwLock::new(HashMap::new())),
            runner: ProcessRunner::new(),
        }
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ExecutionService for SandboxManager {
    async fn create_sandbox(&self, config: SandboxConfig) -> ForgeResult<SandboxId> {
        let id = SandboxId::new();
        let mut sandboxes = self.sandboxes.write().await;
        sandboxes.insert(id.clone(), config);
        tracing::info!(sandbox_id = %id, "created sandbox");
        Ok(id)
    }

    async fn execute(
        &self,
        sandbox_id: &SandboxId,
        command: &str,
        working_dir: Option<&str>,
    ) -> ForgeResult<ExecutionResult> {
        let sandboxes = self.sandboxes.read().await;
        let config = sandboxes
            .get(sandbox_id)
            .ok_or_else(|| ForgeError::NotFound {
                entity: "sandbox".to_string(),
                id: sandbox_id.to_string(),
            })?;

        let timeout = std::time::Duration::from_secs(config.limits.timeout_seconds as u64);

        self.runner
            .run(sandbox_id, command, working_dir, timeout)
            .await
    }

    async fn destroy_sandbox(&self, sandbox_id: &SandboxId) -> ForgeResult<()> {
        let mut sandboxes = self.sandboxes.write().await;
        sandboxes.remove(sandbox_id);
        tracing::info!(sandbox_id = %sandbox_id, "destroyed sandbox");
        Ok(())
    }

    async fn list_sandboxes(&self, project_id: &ProjectId) -> ForgeResult<Vec<SandboxId>> {
        let sandboxes = self.sandboxes.read().await;
        let ids: Vec<SandboxId> = sandboxes
            .iter()
            .filter(|(_, config)| config.project_id == *project_id)
            .map(|(id, _)| id.clone())
            .collect();
        Ok(ids)
    }
}
