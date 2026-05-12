use async_trait::async_trait;

use crate::error::ForgeResult;
use crate::event::ForgeEvent;
use crate::types::*;

/// Core service lifecycle trait
#[async_trait]
pub trait Service: Send + Sync {
    fn name(&self) -> &str;
    async fn start(&self) -> ForgeResult<()>;
    async fn stop(&self) -> ForgeResult<()>;
    async fn health_check(&self) -> ForgeResult<bool>;
}

/// Event bus for inter-service communication
#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, subject: &str, event: ForgeEvent) -> ForgeResult<()>;
    async fn subscribe(&self, subject: &str) -> ForgeResult<Box<dyn EventSubscription>>;
    async fn request(
        &self,
        subject: &str,
        event: ForgeEvent,
        timeout_ms: u64,
    ) -> ForgeResult<ForgeEvent>;
}

/// Subscription handle for receiving events
#[async_trait]
pub trait EventSubscription: Send + Sync {
    async fn next(&mut self) -> ForgeResult<Option<ForgeEvent>>;
    async fn unsubscribe(self: Box<Self>) -> ForgeResult<()>;
}

/// Task graph management
#[async_trait]
pub trait TaskGraphService: Send + Sync {
    async fn create_graph(&self, project_id: &ProjectId, plan: Plan) -> ForgeResult<TaskGraphId>;

    async fn get_tasks(&self, graph_id: &TaskGraphId) -> ForgeResult<Vec<Task>>;

    async fn get_task(&self, task_id: &TaskId) -> ForgeResult<Task>;

    async fn get_ready_tasks(&self, graph_id: &TaskGraphId) -> ForgeResult<Vec<Task>>;

    async fn transition_task(
        &self,
        task_id: &TaskId,
        new_status: TaskStatus,
        reason: &str,
        agent_id: Option<&AgentId>,
    ) -> ForgeResult<Task>;

    async fn add_task(
        &self,
        graph_id: &TaskGraphId,
        task: PlannedTask,
        dependencies: Vec<TaskId>,
    ) -> ForgeResult<Task>;

    async fn get_critical_path(&self, graph_id: &TaskGraphId) -> ForgeResult<Vec<TaskId>>;
}

/// Memory storage and retrieval
#[async_trait]
pub trait MemoryService: Send + Sync {
    async fn store(&self, item: MemoryItem) -> ForgeResult<MemoryItemId>;

    async fn retrieve(
        &self,
        project_id: &ProjectId,
        layer: MemoryLayer,
        key: &str,
    ) -> ForgeResult<Option<MemoryItem>>;

    async fn search(
        &self,
        project_id: &ProjectId,
        query: &str,
        layers: &[MemoryLayer],
        limit: usize,
    ) -> ForgeResult<Vec<MemoryItem>>;

    async fn update(&self, id: &MemoryItemId, content: &str) -> ForgeResult<()>;

    async fn archive(&self, id: &MemoryItemId) -> ForgeResult<()>;

    async fn list_by_layer(
        &self,
        project_id: &ProjectId,
        layer: MemoryLayer,
        limit: usize,
        offset: usize,
    ) -> ForgeResult<Vec<MemoryItem>>;

    async fn store_decision(&self, decision: Decision) -> ForgeResult<String>;

    async fn get_decisions(
        &self,
        project_id: &ProjectId,
        limit: usize,
    ) -> ForgeResult<Vec<Decision>>;
}

/// Model routing and inference
#[async_trait]
pub trait ModelRouter: Send + Sync {
    async fn complete(&self, request: CompletionRequest) -> ForgeResult<CompletionResponse>;

    async fn stream_complete(
        &self,
        request: CompletionRequest,
    ) -> ForgeResult<Box<dyn CompletionStream>>;

    async fn embed(&self, texts: Vec<String>) -> ForgeResult<Vec<Vec<f32>>>;

    async fn get_routing_decision(
        &self,
        request: &CompletionRequest,
    ) -> ForgeResult<RoutingDecision>;
}

/// Streaming completion response
#[async_trait]
pub trait CompletionStream: Send + Sync {
    async fn next_chunk(&mut self) -> ForgeResult<Option<String>>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoutingDecision {
    pub selected_model: String,
    pub tier: ModelTier,
    pub reasoning: String,
    pub estimated_input_tokens: u32,
    pub estimated_output_tokens: u32,
    pub estimated_cost_usd: f64,
}

/// Code indexing and intelligence
#[async_trait]
pub trait CodeIndexService: Send + Sync {
    async fn index_project(&self, root_path: &str) -> ForgeResult<()>;
    async fn index_file(&self, file_path: &str) -> ForgeResult<()>;
    async fn get_symbols(&self, file_path: &str) -> ForgeResult<Vec<Symbol>>;
    async fn find_references(&self, symbol_name: &str) -> ForgeResult<Vec<SymbolReference>>;
    async fn get_dependencies(&self, file_path: &str) -> ForgeResult<Vec<String>>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub file_path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Function,
    Class,
    Method,
    Variable,
    Constant,
    Interface,
    Enum,
    Module,
    Type,
    Import,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolReference {
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub context: String,
}

/// Execution sandbox
#[async_trait]
pub trait ExecutionService: Send + Sync {
    async fn create_sandbox(&self, config: SandboxConfig) -> ForgeResult<SandboxId>;
    async fn execute(
        &self,
        sandbox_id: &SandboxId,
        command: &str,
        working_dir: Option<&str>,
    ) -> ForgeResult<ExecutionResult>;
    async fn destroy_sandbox(&self, sandbox_id: &SandboxId) -> ForgeResult<()>;
    async fn list_sandboxes(&self, project_id: &ProjectId) -> ForgeResult<Vec<SandboxId>>;
}

/// Validation pipeline
#[async_trait]
pub trait ValidationService: Send + Sync {
    async fn run_validation(
        &self,
        project_path: &str,
        checks: &[ValidationType],
    ) -> ForgeResult<ValidationResult>;
}

/// Plugin runtime
#[async_trait]
pub trait PluginRuntime: Send + Sync {
    async fn load_plugin(&self, manifest_path: &str) -> ForgeResult<PluginId>;
    async fn unload_plugin(&self, plugin_id: &PluginId) -> ForgeResult<()>;
    async fn list_plugins(&self) -> ForgeResult<Vec<PluginInfo>>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginInfo {
    pub id: PluginId,
    pub name: String,
    pub version: String,
    pub description: String,
    pub plugin_type: String,
    pub active: bool,
}
