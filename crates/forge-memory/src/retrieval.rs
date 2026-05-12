use forge_core::error::ForgeResult;
use forge_core::types::*;

pub struct ContextAssembler;

#[derive(Debug, Clone)]
pub struct AssembledContext {
    pub items: Vec<ContextItem>,
    pub total_tokens: u32,
    pub omitted_items: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContextItem {
    pub source: String,
    pub content: String,
    pub relevance_score: f64,
    pub token_count: u32,
}

impl ContextAssembler {
    pub fn assemble(
        memory_items: Vec<MemoryItem>,
        max_tokens: u32,
    ) -> ForgeResult<AssembledContext> {
        let mut scored: Vec<(MemoryItem, f64)> = memory_items
            .into_iter()
            .map(|item| {
                let score = Self::relevance_score(&item);
                (item, score)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut items = Vec::new();
        let mut total_tokens = 0u32;
        let mut omitted = Vec::new();

        for (item, score) in scored {
            let token_count = Self::estimate_tokens(&item.content);
            if total_tokens + token_count > max_tokens {
                omitted.push(item.title);
                continue;
            }

            items.push(ContextItem {
                source: format!("{}:{}", item.layer, item.key),
                content: item.content,
                relevance_score: score,
                token_count,
            });
            total_tokens += token_count;
        }

        Ok(AssembledContext {
            items,
            total_tokens,
            omitted_items: omitted,
        })
    }

    fn relevance_score(item: &MemoryItem) -> f64 {
        let base = item.confidence;
        let access_boost = (item.access_count as f64).ln().max(0.0) * 0.1;
        let recency_boost = item
            .last_accessed_at
            .map(|t| {
                let age_hours = (chrono::Utc::now() - t).num_hours() as f64;
                1.0 / (1.0 + age_hours / 24.0)
            })
            .unwrap_or(0.0);

        base + access_boost + recency_boost
    }

    fn estimate_tokens(text: &str) -> u32 {
        (text.len() as f64 / 3.5) as u32
    }
}
