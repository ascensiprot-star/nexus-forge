use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub general: GeneralConfig,
    pub models: ModelsConfig,
    pub agents: AgentsConfig,
    pub memory: MemoryConfig,
    pub execution: ExecutionConfig,
    pub observability: ObservabilityConfig,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            models: ModelsConfig::default(),
            agents: AgentsConfig::default(),
            memory: MemoryConfig::default(),
            execution: ExecutionConfig::default(),
            observability: ObservabilityConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub data_dir: String,
    pub log_level: String,
    pub max_concurrent_tasks: usize,
    pub auto_approve_low_risk: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            data_dir: dirs().unwrap_or_else(|| ".forge".to_string()),
            log_level: "info".to_string(),
            max_concurrent_tasks: 4,
            auto_approve_low_risk: false,
        }
    }
}

fn dirs() -> Option<String> {
    std::env::var("HOME")
        .ok()
        .map(|h| format!("{h}/.nexus-forge"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    pub default_provider: String,
    pub providers: Vec<ModelProviderConfig>,
    pub routing: RoutingConfig,
}

impl Default for ModelsConfig {
    fn default() -> Self {
        Self {
            default_provider: "anthropic".to_string(),
            providers: vec![],
            routing: RoutingConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProviderConfig {
    pub name: String,
    pub api_base: Option<String>,
    pub api_key_env: String,
    pub models: Vec<ModelEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    pub tier: String,
    pub max_context: u32,
    pub max_output: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    pub architecture_design: TaskRoutingRule,
    pub code_implementation: TaskRoutingRule,
    pub code_review: TaskRoutingRule,
    pub test_generation: TaskRoutingRule,
    pub documentation: TaskRoutingRule,
    pub bug_investigation: TaskRoutingRule,
    pub simple_edit: TaskRoutingRule,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            architecture_design: TaskRoutingRule {
                primary_tier: "frontier_reasoning".to_string(),
                fallback_tier: "strong_general".to_string(),
                max_tokens: 16384,
                temperature: 0.3,
            },
            code_implementation: TaskRoutingRule {
                primary_tier: "strong_general".to_string(),
                fallback_tier: "code_specialized".to_string(),
                max_tokens: 8192,
                temperature: 0.1,
            },
            code_review: TaskRoutingRule {
                primary_tier: "strong_general".to_string(),
                fallback_tier: "frontier_reasoning".to_string(),
                max_tokens: 4096,
                temperature: 0.0,
            },
            test_generation: TaskRoutingRule {
                primary_tier: "fast_efficient".to_string(),
                fallback_tier: "strong_general".to_string(),
                max_tokens: 4096,
                temperature: 0.1,
            },
            documentation: TaskRoutingRule {
                primary_tier: "fast_efficient".to_string(),
                fallback_tier: "strong_general".to_string(),
                max_tokens: 8192,
                temperature: 0.2,
            },
            bug_investigation: TaskRoutingRule {
                primary_tier: "strong_general".to_string(),
                fallback_tier: "frontier_reasoning".to_string(),
                max_tokens: 8192,
                temperature: 0.1,
            },
            simple_edit: TaskRoutingRule {
                primary_tier: "fast_efficient".to_string(),
                fallback_tier: "code_specialized".to_string(),
                max_tokens: 2048,
                temperature: 0.0,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRoutingRule {
    pub primary_tier: String,
    pub fallback_tier: String,
    pub max_tokens: u32,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    pub max_revision_cycles: u32,
    pub max_concurrent_agents: usize,
    pub approval_required_for: Vec<String>,
}

impl Default for AgentsConfig {
    fn default() -> Self {
        Self {
            max_revision_cycles: 5,
            max_concurrent_agents: 4,
            approval_required_for: vec![
                "file_delete".to_string(),
                "deployment".to_string(),
                "schema_migration".to_string(),
                "dependency_add".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub max_project_memory_mb: u32,
    pub compaction_interval_hours: u32,
    pub relevance_decay_days: u32,
    pub embedding_model: String,
    pub embedding_dimension: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_project_memory_mb: 500,
            compaction_interval_hours: 168,
            relevance_decay_days: 90,
            embedding_model: "local".to_string(),
            embedding_dimension: 768,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub default_tier: String,
    pub container_runtime: String,
    pub max_concurrent_sandboxes: usize,
    pub default_timeout_seconds: u32,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            default_tier: "standard".to_string(),
            container_runtime: "podman".to_string(),
            max_concurrent_sandboxes: 4,
            default_timeout_seconds: 300,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub log_format: String,
    pub metrics_enabled: bool,
    pub traces_enabled: bool,
    pub event_retention_days: u32,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            log_format: "json".to_string(),
            metrics_enabled: true,
            traces_enabled: true,
            event_retention_days: 30,
        }
    }
}
