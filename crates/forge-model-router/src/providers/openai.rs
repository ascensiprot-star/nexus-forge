use async_trait::async_trait;
use forge_core::error::{ForgeError, ForgeResult};
use forge_core::types::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::ModelProvider;

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    api_base: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_base: "https://api.openai.com".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.api_base = base_url;
        self
    }
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    id: String,
    choices: Vec<Choice>,
    model: String,
    usage: OpenAIUsage,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[async_trait]
impl ModelProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn complete(
        &self,
        model_id: &str,
        messages: &[Message],
        request: &CompletionRequest,
    ) -> ForgeResult<CompletionResponse> {
        let api_messages: Vec<OpenAIMessage> = messages
            .iter()
            .map(|m| OpenAIMessage {
                role: match m.role {
                    MessageRole::System => "system".to_string(),
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                },
                content: m.content.clone(),
            })
            .collect();

        let api_request = OpenAIRequest {
            model: model_id.to_string(),
            messages: api_messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };

        let start = std::time::Instant::now();

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.api_base))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
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

        let api_response: OpenAIResponse =
            response.json().await.map_err(|e| ForgeError::ModelError {
                model_id: model_id.to_string(),
                message: e.to_string(),
            })?;

        let content = api_response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let finish_reason = api_response
            .choices
            .first()
            .and_then(|c| c.finish_reason.clone())
            .unwrap_or_else(|| "stop".to_string());

        Ok(CompletionResponse {
            model_id: api_response.model,
            content,
            input_tokens: api_response.usage.prompt_tokens,
            output_tokens: api_response.usage.completion_tokens,
            latency_ms,
            cost_usd: 0.0,
            finish_reason,
        })
    }

    async fn embed(&self, _model_id: &str, _texts: &[String]) -> ForgeResult<Vec<Vec<f32>>> {
        Err(ForgeError::Internal(
            "OpenAI embedding not yet implemented".to_string(),
        ))
    }
}
