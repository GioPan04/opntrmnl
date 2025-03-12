mod rss;
mod text;
pub use rss::RssPlugin;
pub use text::TextPlugin;

pub trait Plugin {
    async fn device_render(&self) -> Result<String, PluginError>;
}

#[derive(Debug)]
pub enum PluginError {
    Render(askama::Error),
    Http(reqwest::Error),
    Rss(rss_parser::Error),
}
