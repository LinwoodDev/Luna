use std::error::Error;

#[cfg(feature = "handlebars")]
pub mod handlebars;

pub trait TemplateEngine {
    fn render(&self, template: &str, context: &str) -> Result<String, Box<dyn Error>>;
}
