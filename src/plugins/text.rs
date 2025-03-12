use askama::Template;
use serde::{Deserialize, Serialize};

use super::{Plugin, PluginError};

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "./plugins/text.jinja")]
pub struct TextPlugin {
    content: String,
}

impl TextPlugin {
    pub fn new(text: String) -> Self {
        Self { content: text }
    }
}

impl Plugin for TextPlugin {
    async fn device_render(&self) -> Result<String, PluginError> {
        self.render().map_err(|e| PluginError::Render(e))
    }
}
