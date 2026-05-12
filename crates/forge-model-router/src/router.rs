use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use forge_core::error::{ForgeError, ForgeResult};
use forge_core::traits::{CompletionStream, ModelRouter, RoutingDecision};
use forge_core::types::*;
use tokio::sync::RwLock;

use crate::budget::BudgetTracker;
use crate::providers::ModelProvider;
use crate::retry::RetryPolicy;

pub struct ModelRouterService {
    providers: HashMap<String, Arc<dyn ModelProvider>>,
    models: HashMap<String, ModelConfig>,
    tier_map: HashMap<String, Vec<String>>,
    budgets: Arc<RwLock<BudgetTracker>>,
    task_routing: HashMap<String, (String, String)>,
}

impl ModelRouterService {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            models: HashMap::new(),
            tier_map: HashMap::new(),
            budgets: Arc::new(RwLock::new(BudgetTracker::new())),
            task_routing: Self::default_task_routing(),
        }
    }

    pub fn register_provider(&mut self, name: &str, provider: Arc<dyn ModelProvider>) {
        self.providers.insert(name.to_string(), provider);
    }

    pub fn register_model(&mut self, config: ModelConfig) {
        let tier_key = config.tier.to_string();
        self.tier_map
            .entry(tier_key)
            .or_default()
            .push(config.id.clone());
        self.models.insert(config.id.clone(), config);
    }

    fn select_model(&self, task_type: &str, preferred: Option<&str>) -> ForgeResult<&ModelConfig> {
        if let Some(pref) = preferred {
            if let Some(config) = self.models.get(pref) {
                return Ok(config);
            }
        }

        let (primary_tier, fallback_tier) = self
            .task_routing
            .get(task_type)
            .cloned()
            .unwrap_or_else(|| ("strong_general".to_string(), "fast_efficient".to_string()));

        if let Some(models) = self.tier_map.get(&primary_tier) {
            if let Some(model_id) = models.first() {
                if let Some(config) = self.models.get(model_id) {
                    return Ok(config);
                }
            }
        }

        if let Some(models) = self.tier_map.get(&fallback_tier) {
            if let Some(model_id) = models.first() {
                if let Some(config) = self.models.get(model_id) {
                    return Ok(config);
                }
            }
        }

        self.models
            .values()
            .next()
            .ok_or_else(|| ForgeError::ModelError {
                model_id: "none".to_string(),
                message: "no models configured".to_string(),
            })
    }

    fn default_task_routing() -> HashMap<String, (String, String)> {
        let mut map = HashMap::new();
        map.insert(
            "architecture_design".to_string(),
            (
                "frontier_reasoning".to_string(),
                "strong_general".to_string(),
            ),
        );
        map.insert(
            "code_implementation".to_string(),
            ("strong_general".to_string(), "code_specialized".to_string()),
        );
        map.insert(
            "code_review".to_string(),
            (
                "strong_general".to_string(),
                "frontier_reasoning".to_string(),
            ),
        );
        map.insert(
            "test_generation".to_string(),
            ("fast_efficient".to_string(), "strong_general".to_string()),
        );
        map.insert(
            "documentation".to_string(),
            ("fast_efficient".to_string(), "strong_general".to_string()),
        );
        map.insert(
            "bug_investigation".to_string(),
            (
                "strong_general".to_string(),
                "frontier_reasoning".to_string(),
            ),
        );
        map.insert(
            "simple_edit".to_string(),
            ("fast_efficient".to_string(), "code_specialized".to_string()),
        );
        map
    }
}

#[async_trait]
impl ModelRouter for ModelRouterService {
    async fn complete(&self, request: CompletionRequest) -> ForgeResult<CompletionResponse> {
        let model_config =
            self.select_model(&request.task_type, request.preferred_model.as_deref())?;

        let provider =
            self.providers
                .get(&model_config.provider)
                .ok_or_else(|| ForgeError::ModelError {
                    model_id: model_config.id.clone(),
                    message: format!("provider '{}' not found", model_config.provider),
                })?;

        let retry_policy = RetryPolicy::default();
        let mut last_error = None;

        for attempt in 0..retry_policy.max_retries {
            let start = std::time::Instant::now();

            match provider
                .complete(&model_config.id, &request.messages, &request)
                .await
            {
                Ok(response) => {
                    let mut budgets = self.budgets.write().await;
                    budgets.record_usage(
                        response.input_tokens + response.output_tokens,
                        response.cost_usd,
                    );
                    return Ok(response);
                }
                Err(e) => {
                    tracing::warn!(
                        "model call attempt {}/{} failed: {}",
                        attempt + 1,
                        retry_policy.max_retries,
                        e
                    );
                    last_error = Some(e);

                    if attempt < retry_policy.max_retries - 1 {
                        let delay = retry_policy.backoff_ms(attempt);
                        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| ForgeError::ModelError {
            model_id: model_config.id.clone(),
            message: "all retries exhausted".to_string(),
        }))
    }

    async fn stream_complete(
        &self,
        request: CompletionRequest,
    ) -> ForgeResult<Box<dyn CompletionStream>> {
        Err(ForgeError::Internal(
            "streaming not yet implemented".to_string(),
        ))
    }

    async fn embed(&self, texts: Vec<String>) -> ForgeResult<Vec<Vec<f32>>> {
        Err(ForgeError::Internal(
            "embedding not yet implemented via router".to_string(),
        ))
    }

    async fn get_routing_decision(
        &self,
        request: &CompletionRequest,
    ) -> ForgeResult<RoutingDecision> {
        let model_config =
            self.select_model(&request.task_type, request.preferred_model.as_deref())?;

        let est_input = request
            .messages
            .iter()
            .map(|m| m.content.len() as u32 / 4)
            .sum::<u32>();
        let est_output = request.max_tokens.unwrap_or(2048);

        Ok(RoutingDecision {
            selected_model: model_config.id.clone(),
            tier: model_config.tier,
            reasoning: format!(
                "Selected {} (tier {:?}) for task type '{}'",
                model_config.id, model_config.tier, request.task_type
            ),
            estimated_input_tokens: est_input,
            estimated_output_tokens: est_output,
            estimated_cost_usd: (est_input as f64 * model_config.cost_per_input_token)
                + (est_output as f64 * model_config.cost_per_output_token),
        })
    }
}
