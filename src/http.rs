use std::fmt::Display;

use actix_web::{get, web::Html, Responder, ResponseError, Result};

use crate::{
    database::plugin::PluginConfig,
    plugins::{self, Plugin, PluginError},
};

#[derive(Debug)]
struct HttpError;

impl From<PluginError> for HttpError {
    fn from(_: PluginError) -> Self {
        Self
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error")
    }
}

impl ResponseError for HttpError {}

#[get("/plugins/text")]
pub async fn get_text_plugin() -> Result<impl Responder> {
    let text_plugin = plugins::TextPlugin::new(String::from("Hi from opntrmnl!"));
    let plugin_config = PluginConfig::Text(text_plugin);

    Ok(Html::new(
        plugin_config.device_render().map_err(HttpError::from)?,
    ))
}
