use std::collections::HashMap;

pub struct FullTextSearch {
    documents: HashMap<String, (String, String)>,
}

impl FullTextSearch {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    pub fn index(&mut self, id: &str, title: &str, content: &str) {
        self.documents
            .insert(id.to_string(), (title.to_string(), content.to_string()));
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<(String, f64, String)> {
        let query_lower = query.to_lowercase();
        let terms: Vec<&str> = query_lower.split_whitespace().collect();

        let mut results: Vec<(String, f64, String)> = self
            .documents
            .iter()
            .filter_map(|(id, (title, content))| {
                let text_lower = format!("{title} {content}").to_lowercase();
                let match_count = terms.iter().filter(|t| text_lower.contains(*t)).count();

                if match_count > 0 {
                    let score = match_count as f64 / terms.len().max(1) as f64;
                    let snippet = Self::extract_snippet(content, &query_lower);
                    Some((id.clone(), score, snippet))
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        results
    }

    fn extract_snippet(content: &str, query: &str) -> String {
        let lower = content.to_lowercase();
        if let Some(pos) = lower.find(query) {
            let start = pos.saturating_sub(50);
            let end = (pos + query.len() + 50).min(content.len());
            let snippet = &content[start..end];
            format!("...{snippet}...")
        } else {
            content.chars().take(150).collect::<String>()
        }
    }

    pub fn remove(&mut self, id: &str) {
        self.documents.remove(id);
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}

impl Default for FullTextSearch {
    fn default() -> Self {
        Self::new()
    }
}
