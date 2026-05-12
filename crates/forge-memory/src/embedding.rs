use forge_core::error::ForgeResult;

pub struct EmbeddingGenerator {
    dimension: usize,
}

impl EmbeddingGenerator {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Generate a simple hash-based embedding for local/offline use.
    /// In production, this delegates to a real embedding model.
    pub fn generate_local(&self, text: &str) -> ForgeResult<Vec<f32>> {
        let mut embedding = vec![0.0f32; self.dimension];
        let bytes = text.as_bytes();

        for (i, &b) in bytes.iter().enumerate() {
            let idx = i % self.dimension;
            embedding[idx] += (b as f32 - 128.0) / 128.0;
        }

        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for v in &mut embedding {
                *v /= magnitude;
            }
        }

        Ok(embedding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_generation() {
        let gen = EmbeddingGenerator::new(128);
        let emb = gen.generate_local("hello world").unwrap();
        assert_eq!(emb.len(), 128);

        let magnitude: f32 = emb.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.01);
    }
}
