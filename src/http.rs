use actix_web::{get, web::Html, HttpResponse, Responder};

use crate::{
    database::plugin::PluginConfig,
    plugins::{self, Plugin},
};

#[get("/plugins/text")]
pub async fn get_text_plugin() -> impl Responder {
    let text_plugin = plugins::TextPlugin::new(String::from("Hi from opntrmnl!"));
    let plugin_config = PluginConfig::Text(text_plugin);

    Html::new(plugin_config.device_render().unwrap())
}
