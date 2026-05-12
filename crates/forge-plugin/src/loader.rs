use std::path::Path;

use forge_core::error::{ForgeError, ForgeResult};

use crate::manifest::PluginManifest;

pub fn load_manifest(path: &Path) -> ForgeResult<PluginManifest> {
    let content = std::fs::read_to_string(path)?;

    if path.extension().and_then(|e| e.to_str()) == Some("toml") {
        toml::from_str(&content).map_err(|e| ForgeError::PluginError {
            plugin_id: path.display().to_string(),
            message: format!("invalid manifest: {e}"),
        })
    } else {
        serde_json::from_str(&content).map_err(|e| ForgeError::PluginError {
            plugin_id: path.display().to_string(),
            message: format!("invalid manifest: {e}"),
        })
    }
}
