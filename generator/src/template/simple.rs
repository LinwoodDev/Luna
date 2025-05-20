use crate::template::Error;
use serde::Serialize;

use super::TemplateEngine;

pub struct SimpleTemplateEngine;

impl<T : Serialize> TemplateEngine<T> for SimpleTemplateEngine {
    fn render(&self, template: &str, _context: &T) -> Result<String, Box<dyn Error>> {
        Ok(template.to_owned())
    }
}
