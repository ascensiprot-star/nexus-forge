use std::collections::HashMap;

use forge_core::error::ForgeResult;

pub struct LocalVectorStore {
    dimension: usize,
    vectors: HashMap<String, Vec<f32>>,
    metadata: HashMap<String, serde_json::Value>,
}

impl LocalVectorStore {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            vectors: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        id: &str,
        vector: Vec<f32>,
        meta: serde_json::Value,
    ) -> ForgeResult<()> {
        assert_eq!(vector.len(), self.dimension);
        self.vectors.insert(id.to_string(), vector);
        self.metadata.insert(id.to_string(), meta);
        Ok(())
    }

    pub fn search(&self, query: &[f32], limit: usize) -> Vec<(String, f32)> {
        let mut scores: Vec<(String, f32)> = self
            .vectors
            .iter()
            .map(|(id, vec)| {
                let score = cosine_similarity(query, vec);
                (id.clone(), score)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(limit);
        scores
    }

    pub fn remove(&mut self, id: &str) {
        self.vectors.remove(id);
        self.metadata.remove(id);
    }

    pub fn len(&self) -> usize {
        self.vectors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vectors.is_empty()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }

    dot / (mag_a * mag_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut store = LocalVectorStore::new(3);
        store
            .insert("a", vec![1.0, 0.0, 0.0], serde_json::json!({"name": "a"}))
            .unwrap();
        store
            .insert("b", vec![0.0, 1.0, 0.0], serde_json::json!({"name": "b"}))
            .unwrap();

        let results = store.search(&[1.0, 0.0, 0.0], 2);
        assert_eq!(results[0].0, "a");
        assert!((results[0].1 - 1.0).abs() < 0.001);
    }
}
