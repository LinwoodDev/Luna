use crate::template::Error;
use serde::Serialize;

use super::TemplateEngine;

pub struct SimpleTemplateEngine;

impl TemplateEngine for SimpleTemplateEngine {
    fn render<T: Serialize>(&self, template: &str, _context: &T) -> Result<String, Box<dyn Error>> {
        Ok(template.to_owned())
    }
}
