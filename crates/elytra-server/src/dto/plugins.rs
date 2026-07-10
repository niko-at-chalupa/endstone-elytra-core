use serde::{Deserialize, Serialize};
use elytra_core::types::Plugin;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginResponse {
    pub name: String,
    pub kebabbed_name: String,
    pub repository_url: String,
}

impl PluginResponse {
    pub fn from_plugin(plugin: &dyn Plugin) -> Self {
        Self {
            name: plugin.name().to_string(),
            kebabbed_name: plugin.kebabbed_name().to_string(),
            repository_url: plugin.repository_url().to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ListPluginsQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Debug)]
pub struct ListPluginsResponse {
    pub plugins: Vec<PluginResponse>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

const DEFAULT_LIMIT: usize = 20;
const MAX_LIMIT: usize = 100;