use serde::{Deserialize, Serialize};

use crate::plugins::{Plugin, PluginError, RssPlugin, TextPlugin};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "plugin_type", content = "config", rename_all = "snake_case")]
pub enum PluginConfig {
    Text(TextPlugin),
    Rss(RssPlugin),
}

impl Plugin for PluginConfig {
    async fn device_render(&self) -> Result<String, PluginError> {
        match self {
            Self::Text(p) => p.device_render().await,
            Self::Rss(p) => p.device_render().await,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistPlugin {
    pub playlist_id: usize,
    #[serde(flatten)]
    pub config: PluginConfig,
}
