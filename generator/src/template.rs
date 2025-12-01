use std::error::Error;

use serde::Serialize;

#[cfg(feature = "handlebars")]
pub mod handlebars;
pub mod simple;

pub trait TemplateEngine {
    fn render<T: Serialize>(&self, template: &str, context: &T) -> Result<String, Box<dyn Error + Send + Sync>>;
}
