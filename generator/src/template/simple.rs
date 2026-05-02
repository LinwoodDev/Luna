use serde::Serialize;
use std::error::Error;

use super::TemplateEngine;

pub struct SimpleTemplateEngine;

impl TemplateEngine for SimpleTemplateEngine {
    fn render<T: Serialize>(
        &self,
        template: &str,
        _context: &T,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(template.to_owned())
    }
}
