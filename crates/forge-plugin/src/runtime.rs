use std::collections::HashMap;

use async_trait::async_trait;
use forge_core::error::{ForgeError, ForgeResult};
use forge_core::traits::{PluginInfo, PluginRuntime};
use forge_core::types::PluginId;

use crate::manifest::PluginManifest;

pub struct PluginManager {
    plugins: HashMap<PluginId, LoadedPlugin>,
}

struct LoadedPlugin {
    id: PluginId,
    manifest: PluginManifest,
    active: bool,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PluginRuntime for PluginManager {
    async fn load_plugin(&self, manifest_path: &str) -> ForgeResult<PluginId> {
        let path = std::path::Path::new(manifest_path);
        let manifest = crate::loader::load_manifest(path)?;

        let id = PluginId::new();
        tracing::info!(plugin_id = %id, name = %manifest.name, "loaded plugin");
        Ok(id)
    }

    async fn unload_plugin(&self, plugin_id: &PluginId) -> ForgeResult<()> {
        tracing::info!(plugin_id = %plugin_id, "unloaded plugin");
        Ok(())
    }

    async fn list_plugins(&self) -> ForgeResult<Vec<PluginInfo>> {
        let infos = self
            .plugins
            .values()
            .map(|p| PluginInfo {
                id: p.id.clone(),
                name: p.manifest.name.clone(),
                version: p.manifest.version.clone(),
                description: p.manifest.description.clone(),
                plugin_type: format!("{:?}", p.manifest.plugin_type),
                active: p.active,
            })
            .collect();
        Ok(infos)
    }
}
