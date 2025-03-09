use serde::{Deserialize, Serialize};

use crate::plugins::{Plugin, PluginError, TextPlugin};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "plugin_type", content = "config", rename_all = "snake_case")]
pub enum PluginConfig {
    Text(TextPlugin),
}

impl Plugin for PluginConfig {
    fn device_render(&self) -> Result<String, PluginError> {
        match self {
            Self::Text(p) => p.device_render(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistPlugin {
    pub playlist_id: usize,
    #[serde(flatten)]
    pub config: PluginConfig,
}
