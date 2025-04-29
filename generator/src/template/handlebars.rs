use std::error::Error;

use handlebars::Handlebars;

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

impl TemplateEngine for HandlebarsTemplateEngine<'_> {
    fn render(&self, template: &str, context: &str) -> Result<String, Box<dyn Error>> {
        let rendered = self.registry.render(template, &context.to_owned())?;
        Ok(rendered)
    }
}
