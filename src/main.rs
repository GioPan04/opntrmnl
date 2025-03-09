use database::plugin::PluginConfig;
use plugins::Plugin;

mod database;
mod plugins;

fn main() {
    let text_plugin = plugins::TextPlugin::new(String::from("trmnl"));
    let plugin_config = PluginConfig::Text(text_plugin);

    // println!("{}", serde_json::to_string_pretty(&plugin_config).unwrap());
    println!("{}", plugin_config.device_render().unwrap());
}
