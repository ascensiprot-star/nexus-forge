use forge_core::error::ForgeResult;
use forge_core::types::*;
use serde::{Deserialize, Serialize};

use crate::full_text::FullTextSearch;

pub struct SearchService {
    fts: FullTextSearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: String,
    pub scope: SearchScope,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchScope {
    Code,
    Memory,
    Decision,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub source: String,
    pub title: String,
    pub snippet: String,
    pub score: f64,
    pub file_path: Option<String>,
    pub line_number: Option<u32>,
    pub metadata: serde_json::Value,
}

impl SearchService {
    pub fn new() -> Self {
        Self {
            fts: FullTextSearch::new(),
        }
    }

    pub fn index_document(&mut self, id: &str, title: &str, content: &str) {
        self.fts.index(id, title, content);
    }

    pub fn search(&self, query: &SearchQuery) -> ForgeResult<Vec<SearchResult>> {
        let matches = self.fts.search(&query.text, query.limit);

        let results = matches
            .into_iter()
            .map(|(id, score, snippet)| SearchResult {
                source: id,
                title: String::new(),
                snippet,
                score,
                file_path: None,
                line_number: None,
                metadata: serde_json::json!({}),
            })
            .collect();

        Ok(results)
    }
}

impl Default for SearchService {
    fn default() -> Self {
        Self::new()
    }
}
