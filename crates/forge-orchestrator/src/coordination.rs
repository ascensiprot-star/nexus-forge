use forge_core::types::{AgentId, AgentRole, TaskId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: String,
    pub from_agent: AgentId,
    pub to_agent: AgentMessageTarget,
    pub message_type: AgentMessageType,
    pub task_id: TaskId,
    pub payload: serde_json::Value,
    pub priority: MessagePriority,
    pub requires_response: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentMessageTarget {
    Agent(AgentId),
    Role(AgentRole),
    Broadcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentMessageType {
    TaskAssignment,
    TaskUpdate,
    ContextRequest,
    ContextResponse,
    ReviewRequest,
    ReviewResponse,
    ApprovalRequest,
    ApprovalResponse,
    CollaborationNote,
    Escalation,
    ArtifactProduced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}
