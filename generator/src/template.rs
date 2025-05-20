use std::error::Error;

use serde::Serialize;

pub mod simple;
#[cfg(feature = "handlebars")]
pub mod handlebars;

pub trait TemplateEngine<T : Serialize>  {
    fn render(&self, template: &str, context: &T) -> Result<String, Box<dyn Error>>;
}
