use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Identifiers ──

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskGraphId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryItemId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApprovalId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PluginId(pub String);

macro_rules! impl_id {
    ($name:ident) => {
        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4().to_string())
            }

            pub fn from_str(s: &str) -> Self {
                Self(s.to_string())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

impl_id!(ProjectId);
impl_id!(TaskId);
impl_id!(TaskGraphId);
impl_id!(AgentId);
impl_id!(UserId);
impl_id!(MemoryItemId);
impl_id!(ApprovalId);
impl_id!(SandboxId);
impl_id!(PluginId);

// ── Project ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub root_path: String,
    pub status: ProjectStatus,
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Empty,
    Planning,
    Building,
    Review,
    Ready,
    Deployed,
    Paused,
    Error,
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Empty => "empty",
            Self::Planning => "planning",
            Self::Building => "building",
            Self::Review => "review",
            Self::Ready => "ready",
            Self::Deployed => "deployed",
            Self::Paused => "paused",
            Self::Error => "error",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for ProjectStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "empty" => Ok(Self::Empty),
            "planning" => Ok(Self::Planning),
            "building" => Ok(Self::Building),
            "review" => Ok(Self::Review),
            "ready" => Ok(Self::Ready),
            "deployed" => Ok(Self::Deployed),
            "paused" => Ok(Self::Paused),
            "error" => Ok(Self::Error),
            _ => Err(format!("unknown project status: {s}")),
        }
    }
}

// ── Task ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub graph_id: TaskGraphId,
    pub parent_id: Option<TaskId>,
    pub title: String,
    pub description: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub priority: u8,
    pub assigned_agent: Option<AgentId>,
    pub estimated_effort: Option<EffortEstimate>,
    pub actual_effort_minutes: Option<u32>,
    pub inputs: Vec<TaskInput>,
    pub outputs: Vec<TaskOutput>,
    pub constraints: Vec<String>,
    pub dependencies: Vec<TaskId>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Epic,
    Feature,
    Task,
    Subtask,
    BugFix,
    Refactor,
    Investigation,
    Review,
    Test,
    Documentation,
    Deployment,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Assigned,
    InProgress,
    Validating,
    Reviewing,
    Revising,
    Complete,
    Deployed,
    Failed,
    Blocked,
    Cancelled,
    Deferred,
}

impl TaskStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Complete | Self::Deployed | Self::Failed | Self::Cancelled
        )
    }

    pub fn can_transition_to(&self, target: &TaskStatus) -> bool {
        use TaskStatus::*;
        matches!(
            (self, target),
            (Pending, Assigned)
                | (Pending, Cancelled)
                | (Pending, Deferred)
                | (Assigned, InProgress)
                | (Assigned, Cancelled)
                | (InProgress, Validating)
                | (InProgress, Blocked)
                | (InProgress, Cancelled)
                | (InProgress, Failed)
                | (Validating, Reviewing)
                | (Validating, Revising)
                | (Validating, Failed)
                | (Reviewing, Complete)
                | (Reviewing, Revising)
                | (Reviewing, Cancelled)
                | (Revising, InProgress)
                | (Revising, Failed)
                | (Blocked, InProgress)
                | (Blocked, Cancelled)
                | (Complete, Deployed)
                | (Deferred, Pending)
        )
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default();
        write!(f, "{s}")
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|e| format!("unknown task status '{s}': {e}"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffortEstimate {
    pub optimistic_minutes: u32,
    pub expected_minutes: u32,
    pub pessimistic_minutes: u32,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInput {
    pub name: String,
    pub source: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskOutput {
    pub name: String,
    pub artifact_type: String,
    pub path: Option<String>,
}

// ── Agent ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: AgentId,
    pub name: String,
    pub role: AgentRole,
    pub status: AgentStatus,
    pub current_task: Option<TaskId>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRole {
    AiEngineer,
    SoftwareEngineer,
    DataEngineer,
    BackendArchitect,
    FrontendEngineer,
    DevopsEngineer,
    SecurityEngineer,
    QaEngineer,
    DatabaseArchitect,
    PerformanceEngineer,
    DocumentationEngineer,
    CodeReviewer,
    ProductStrategist,
    UxResearcher,
    TechnicalWriter,
    InfrastructureEngineer,
    MlOpsEngineer,
    IntegrationEngineer,
    AgentSupervisor,
    BuildReleaseEngineer,
}

impl std::fmt::Display for AgentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Idle,
    Preparing,
    Thinking,
    Acting,
    Observing,
    Reflecting,
    WaitingApproval,
    Blocked,
    Done,
    Error,
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default();
        write!(f, "{s}")
    }
}

// ── Memory ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryLayer {
    Working,
    Project,
    Codebase,
    Decision,
    Agent,
    UserPreference,
    SemanticRetrieval,
}

impl std::fmt::Display for MemoryLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default();
        write!(f, "{s}")
    }
}

impl std::str::FromStr for MemoryLayer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|e| format!("unknown memory layer '{s}': {e}"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub id: MemoryItemId,
    pub project_id: ProjectId,
    pub layer: MemoryLayer,
    pub item_type: String,
    pub key: String,
    pub title: String,
    pub content: String,
    pub source: String,
    pub confidence: f64,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub access_count: u64,
    pub last_accessed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub project_id: ProjectId,
    pub decision_type: String,
    pub title: String,
    pub question: String,
    pub chosen_option: String,
    pub rejected_options: Vec<String>,
    pub reasoning: String,
    pub tradeoffs: Vec<String>,
    pub constraints: Vec<String>,
    pub decided_by: String,
    pub task_id: Option<TaskId>,
    pub supersedes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── Model ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelTier {
    FrontierReasoning,
    StrongGeneral,
    FastEfficient,
    CodeSpecialized,
    LongContext,
    LocalOffline,
}

impl std::fmt::Display for ModelTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub tier: ModelTier,
    pub max_context_tokens: u32,
    pub max_output_tokens: u32,
    pub cost_per_input_token: f64,
    pub cost_per_output_token: f64,
    pub supports_streaming: bool,
    pub supports_tools: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub task_type: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
    pub preferred_model: Option<String>,
    pub budget: Option<TokenBudget>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub model_id: String,
    pub content: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub latency_ms: u64,
    pub cost_usd: f64,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBudget {
    pub total_budget: u32,
    pub consumed: u32,
    pub per_call_limit: u32,
    pub input_budget_ratio: f64,
    pub escalation_threshold: f64,
    pub hard_stop: u32,
}

// ── Execution ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxTier {
    Light,
    Standard,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub project_id: ProjectId,
    pub tier: SandboxTier,
    pub base_image: Option<String>,
    pub mount_paths: Vec<String>,
    pub output_dir: String,
    pub limits: ResourceLimits,
    pub network_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_cores: u32,
    pub memory_mb: u32,
    pub disk_mb: u32,
    pub timeout_seconds: u32,
    pub max_processes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub sandbox_id: SandboxId,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub artifacts: Vec<String>,
}

// ── Approval ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: ApprovalId,
    pub agent_id: AgentId,
    pub task_id: TaskId,
    pub action_description: String,
    pub risk_level: RiskLevel,
    pub options: Vec<String>,
    pub status: ApprovalStatus,
    pub response_comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

// ── Intent ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentRequest {
    pub project_id: ProjectId,
    pub user_id: UserId,
    pub intent_text: String,
    pub file_references: Vec<String>,
    pub context: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentResponse {
    pub plan_id: String,
    pub plan: Plan,
    pub questions: Vec<ClarifyingQuestion>,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub title: String,
    pub description: String,
    pub tasks: Vec<PlannedTask>,
    pub estimated_total_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedTask {
    pub title: String,
    pub description: String,
    pub task_type: TaskType,
    pub priority: u8,
    pub agent_role: AgentRole,
    pub dependencies: Vec<String>,
    pub estimated_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClarifyingQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub required: bool,
}

// ── Validation ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub passed: bool,
    pub checks: Vec<ValidationCheck>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    pub name: String,
    pub check_type: ValidationType,
    pub passed: bool,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationType {
    Lint,
    TypeCheck,
    UnitTest,
    IntegrationTest,
    SecurityScan,
    AcceptanceCheck,
}

// ── Auth ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Developer,
    Reviewer,
    Viewer,
}
