use askama::Template;
use rss_parser::Channel;
use serde::{Deserialize, Serialize};

use super::{Plugin, PluginError};

#[derive(Debug, Serialize, Deserialize)]
pub struct RssPlugin {
    pub endpoint_url: String,
}

#[derive(Debug, Template)]
#[template(path = "./plugins/rss.jinja")]
struct RssTemplate {
    channel: Channel,
}

impl Plugin for RssPlugin {
    async fn device_render(&self) -> Result<String, super::PluginError> {
        let content = reqwest::get(&self.endpoint_url)
            .await
            .map_err(|e| PluginError::Http(e))?
            .bytes()
            .await
            .map_err(|e| PluginError::Http(e))?;

        let channel = Channel::read_from(&content[..]).map_err(|e| PluginError::Rss(e))?;

        RssTemplate { channel }
            .render()
            .map_err(|e| PluginError::Render(e))
    }
}
