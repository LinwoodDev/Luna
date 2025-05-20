use std::error::Error;

use handlebars::Handlebars;
use serde::Serialize;

use super::TemplateEngine;

pub struct HandlebarsTemplateEngine<'a> {
    registry: Handlebars<'a>,
}

impl HandlebarsTemplateEngine<'_> {
    pub fn new() -> Self {
        let mut registry = Handlebars::new();
        registry.set_strict_mode(true);
        HandlebarsTemplateEngine { registry }
    }
}

impl<T: Serialize> TemplateEngine<T> for HandlebarsTemplateEngine<'_> {
    fn render(&self, template: &str, context: &T) -> Result<String, Box<dyn Error>> {
        let rendered = self.registry.render(template, &context.to_owned())?;
        Ok(rendered)
    }
}
