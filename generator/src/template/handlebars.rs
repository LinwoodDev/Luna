use std::error::Error;

use handlebars::Handlebars;
use serde::Serialize;

use super::TemplateEngine;

pub struct HandlebarsTemplateEngine<'a> {
    registry: Handlebars<'a>,
}

impl<'a> HandlebarsTemplateEngine<'a> {
    pub fn new() -> Self {
        let mut registry = Handlebars::new();
        registry.set_strict_mode(true);
        HandlebarsTemplateEngine { registry }
    }

    pub fn registry(&mut self) -> &mut Handlebars<'a> {
        &mut self.registry
    }
}

impl<'a> Default for HandlebarsTemplateEngine<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine for HandlebarsTemplateEngine<'_> {
    fn render<T: Serialize>(&self, template: &str, context: &T) -> Result<String, Box<dyn Error + Send + Sync>> {
        let rendered = self.registry.render(template, &context)?;
        Ok(rendered)
    }
}
