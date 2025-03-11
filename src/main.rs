use actix_web::{App, HttpServer};

mod database;
mod http;
mod plugins;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(http::get_text_plugin))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

    // println!("{}", serde_json::to_string_pretty(&plugin_config).unwrap());
    // println!("{}", plugin_config.device_render().unwrap());
}
