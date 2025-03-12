use askama::Template;

use super::{Plugin, PluginError};

#[derive(Debug, Template)]
#[template(path = "./plugins/error.jinja")]
pub struct ErrorPlugin {
    pub error: PluginError,
}

impl Plugin for ErrorPlugin {
    async fn device_render(&self) -> Result<String, PluginError> {
        let _ = format!("{:?}", self.error);
        self.render().map_err(|e| PluginError::Render(e))
    }
}
