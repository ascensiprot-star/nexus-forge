use forge_core::error::ForgeResult;

pub struct SemanticSearch {
    dimension: usize,
}

impl SemanticSearch {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }
}
