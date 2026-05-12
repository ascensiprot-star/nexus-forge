pub mod anthropic;
pub mod openai;

use async_trait::async_trait;
use forge_core::error::ForgeResult;
use forge_core::types::*;

#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn name(&self) -> &str;

    async fn complete(
        &self,
        model_id: &str,
        messages: &[Message],
        request: &CompletionRequest,
    ) -> ForgeResult<CompletionResponse>;

    async fn embed(&self, model_id: &str, texts: &[String]) -> ForgeResult<Vec<Vec<f32>>>;
}
