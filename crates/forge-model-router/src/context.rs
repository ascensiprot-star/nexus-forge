pub struct ContextCompressor;

impl ContextCompressor {
    pub fn estimate_tokens(text: &str) -> u32 {
        (text.len() as f64 / 3.5) as u32
    }

    pub fn truncate_to_budget(text: &str, max_tokens: u32) -> String {
        let max_chars = (max_tokens as f64 * 3.5) as usize;
        if text.len() <= max_chars {
            return text.to_string();
        }

        let truncated = &text[..max_chars];
        if let Some(last_newline) = truncated.rfind('\n') {
            truncated[..last_newline].to_string() + "\n[...truncated...]"
        } else {
            truncated.to_string() + "[...truncated...]"
        }
    }

    pub fn prioritize_context(items: &mut Vec<(String, f64)>, max_tokens: u32) -> Vec<String> {
        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut result = Vec::new();
        let mut tokens_used = 0u32;

        for (content, _score) in items {
            let tokens = Self::estimate_tokens(content);
            if tokens_used + tokens > max_tokens {
                break;
            }
            result.push(content.clone());
            tokens_used += tokens;
        }

        result
    }
}
