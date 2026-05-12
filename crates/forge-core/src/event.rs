use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeEvent {
    pub id: String,
    pub event_type: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: String,
    pub payload: serde_json::Value,
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub project_id: ProjectId,
    pub user_id: Option<UserId>,
    pub agent_id: Option<AgentId>,
    pub task_id: Option<TaskId>,
}

impl ForgeEvent {
    pub fn new(
        event_type: impl Into<String>,
        source: impl Into<String>,
        payload: serde_json::Value,
        metadata: EventMetadata,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type: event_type.into(),
            source: source.into(),
            timestamp: Utc::now(),
            correlation_id: Uuid::new_v4().to_string(),
            payload,
            metadata,
        }
    }

    pub fn with_correlation(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = correlation_id.into();
        self
    }
}

// ── Typed Event Payloads ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCreatedPayload {
    pub task_id: TaskId,
    pub graph_id: TaskGraphId,
    pub title: String,
    pub task_type: TaskType,
    pub priority: u8,
    pub dependencies: Vec<TaskId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTransitionPayload {
    pub task_id: TaskId,
    pub from_status: TaskStatus,
    pub to_status: TaskStatus,
    pub agent_id: Option<AgentId>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletedPayload {
    pub task_id: TaskId,
    pub outputs: Vec<TaskOutput>,
    pub duration_ms: u64,
    pub model_tokens_used: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAssignedPayload {
    pub agent_id: AgentId,
    pub task_id: TaskId,
    pub estimated_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActionPayload {
    pub agent_id: AgentId,
    pub task_id: TaskId,
    pub action_type: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEscalationPayload {
    pub agent_id: AgentId,
    pub task_id: TaskId,
    pub escalation_type: String,
    pub description: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInvocationPayload {
    pub model_id: String,
    pub tier: ModelTier,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub latency_ms: u64,
    pub cost_usd: f64,
    pub task_id: TaskId,
    pub agent_id: AgentId,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangedPayload {
    pub path: String,
    pub change_type: FileChangeType,
    pub agent_id: Option<AgentId>,
    pub task_id: Option<TaskId>,
    pub diff_summary: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileChangeType {
    Created,
    Modified,
    Deleted,
    Renamed,
}

// ── Event Type Constants ──

pub mod event_types {
    pub const TASK_CREATED: &str = "forge.task.created";
    pub const TASK_TRANSITIONED: &str = "forge.task.transitioned";
    pub const TASK_COMPLETED: &str = "forge.task.completed";
    pub const AGENT_ASSIGNED: &str = "forge.agent.assigned";
    pub const AGENT_ACTION: &str = "forge.agent.action";
    pub const AGENT_ESCALATION: &str = "forge.agent.escalation";
    pub const MODEL_INVOCATION: &str = "forge.model.invocation";
    pub const FILE_CHANGED: &str = "forge.file.changed";
    pub const MEMORY_STORED: &str = "forge.memory.stored";
    pub const EXECUTION_STARTED: &str = "forge.execution.started";
    pub const EXECUTION_COMPLETED: &str = "forge.execution.completed";
    pub const SYSTEM_ERROR: &str = "forge.system.error";
}
