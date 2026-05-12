use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub auto_approve_low_risk: bool,
    pub max_concurrent_agents: usize,
    pub preferred_languages: Vec<String>,
    pub code_conventions: Vec<String>,
    pub excluded_paths: Vec<String>,
    pub model_overrides: serde_json::Value,
}
