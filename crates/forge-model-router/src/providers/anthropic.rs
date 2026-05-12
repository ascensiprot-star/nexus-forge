use async_trait::async_trait;
use forge_core::error::{ForgeError, ForgeResult};
use forge_core::types::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::ModelProvider;

pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    api_base: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_base: "https://api.anthropic.com".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.api_base = base_url;
        self
    }
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    id: String,
    content: Vec<ContentBlock>,
    model: String,
    usage: Usage,
    stop_reason: Option<String>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[async_trait]
impl ModelProvider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    async fn complete(
        &self,
        model_id: &str,
        messages: &[Message],
        request: &CompletionRequest,
    ) -> ForgeResult<CompletionResponse> {
        let system_msg = messages
            .iter()
            .find(|m| m.role == MessageRole::System)
            .map(|m| m.content.clone());

        let api_messages: Vec<AnthropicMessage> = messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| AnthropicMessage {
                role: match m.role {
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                    MessageRole::System => unreachable!(),
                },
                content: m.content.clone(),
            })
            .collect();

        let api_request = AnthropicRequest {
            model: model_id.to_string(),
            max_tokens: request.max_tokens.unwrap_or(4096),
            messages: api_messages,
            system: system_msg,
            temperature: request.temperature,
        };

        let start = std::time::Instant::now();

        let response = self
            .client
            .post(format!("{}/v1/messages", self.api_base))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&api_request)
            .send()
            .await
            .map_err(|e| ForgeError::ModelError {
                model_id: model_id.to_string(),
                message: e.to_string(),
            })?;

        let latency_ms = start.elapsed().as_millis() as u64;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ForgeError::ModelError {
                model_id: model_id.to_string(),
                message: format!("HTTP {status}: {body}"),
            });
        }

        let api_response: AnthropicResponse =
            response.json().await.map_err(|e| ForgeError::ModelError {
                model_id: model_id.to_string(),
                message: e.to_string(),
            })?;

        let content = api_response
            .content
            .iter()
            .filter_map(|b| b.text.as_ref())
            .cloned()
            .collect::<Vec<_>>()
            .join("");

        let input_tokens = api_response.usage.input_tokens;
        let output_tokens = api_response.usage.output_tokens;

        Ok(CompletionResponse {
            model_id: api_response.model,
            content,
            input_tokens,
            output_tokens,
            latency_ms,
            cost_usd: 0.0, // calculated by router based on model config
            finish_reason: api_response
                .stop_reason
                .unwrap_or_else(|| "stop".to_string()),
        })
    }

    async fn embed(&self, _model_id: &str, _texts: &[String]) -> ForgeResult<Vec<Vec<f32>>> {
        Err(ForgeError::Internal(
            "Anthropic does not support embeddings directly".to_string(),
        ))
    }
}
