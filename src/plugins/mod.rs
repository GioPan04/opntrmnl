mod text;
pub use text::TextPlugin;

pub trait Plugin {
    fn device_render(&self) -> Result<String, PluginError>;
}

#[derive(Debug)]
pub enum PluginError {
    Render(rinja::Error),
}
