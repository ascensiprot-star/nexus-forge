use thiserror::Error;

pub type ForgeResult<T> = Result<T, ForgeError>;

#[derive(Error, Debug)]
pub enum ForgeError {
    #[error("not found: {entity} with id '{id}'")]
    NotFound { entity: String, id: String },

    #[error("already exists: {entity} with key '{key}'")]
    AlreadyExists { entity: String, key: String },

    #[error("invalid state transition: {from} -> {to} for {entity}")]
    InvalidTransition {
        entity: String,
        from: String,
        to: String,
    },

    #[error("validation failed: {message}")]
    ValidationError { message: String },

    #[error("permission denied: {action} on {resource}")]
    PermissionDenied { action: String, resource: String },

    #[error("approval required: {description}")]
    ApprovalRequired { description: String },

    #[error("budget exceeded: {details}")]
    BudgetExceeded { details: String },

    #[error("agent error: [{agent_id}] {message}")]
    AgentError { agent_id: String, message: String },

    #[error("model error: [{model_id}] {message}")]
    ModelError { model_id: String, message: String },

    #[error("execution error: {message}")]
    ExecutionError { message: String },

    #[error("sandbox error: {message}")]
    SandboxError { message: String },

    #[error("database error: {0}")]
    Database(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("configuration error: {0}")]
    Configuration(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("timeout: {operation} exceeded {timeout_ms}ms")]
    Timeout { operation: String, timeout_ms: u64 },

    #[error("dependency cycle detected involving task '{task_id}'")]
    CyclicDependency { task_id: String },

    #[error("plugin error: [{plugin_id}] {message}")]
    PluginError { plugin_id: String, message: String },
}

impl From<serde_json::Error> for ForgeError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}

#[cfg(feature = "sqlite")]
impl From<rusqlite::Error> for ForgeError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Database(e.to_string())
    }
}
