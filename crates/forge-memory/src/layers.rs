use forge_core::types::MemoryLayer;

pub struct LayerConfig {
    pub layer: MemoryLayer,
    pub max_items: usize,
    pub ttl_days: Option<u32>,
    pub auto_compact: bool,
}

pub fn default_layer_configs() -> Vec<LayerConfig> {
    vec![
        LayerConfig {
            layer: MemoryLayer::Working,
            max_items: 100,
            ttl_days: Some(1),
            auto_compact: false,
        },
        LayerConfig {
            layer: MemoryLayer::Project,
            max_items: 10_000,
            ttl_days: None,
            auto_compact: true,
        },
        LayerConfig {
            layer: MemoryLayer::Codebase,
            max_items: 50_000,
            ttl_days: None,
            auto_compact: true,
        },
        LayerConfig {
            layer: MemoryLayer::Decision,
            max_items: 5_000,
            ttl_days: None,
            auto_compact: false,
        },
        LayerConfig {
            layer: MemoryLayer::Agent,
            max_items: 10_000,
            ttl_days: None,
            auto_compact: true,
        },
        LayerConfig {
            layer: MemoryLayer::UserPreference,
            max_items: 1_000,
            ttl_days: None,
            auto_compact: false,
        },
        LayerConfig {
            layer: MemoryLayer::SemanticRetrieval,
            max_items: 100_000,
            ttl_days: None,
            auto_compact: true,
        },
    ]
}
