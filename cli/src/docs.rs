use std::{fs, path::Path, io};

use handlebars::{Handlebars, TemplateError, Context};
use luna_api::models::RepositoryData;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocsError {
    #[error("Template invalid: {0}")]
    TemplateError(#[from] TemplateError),
    #[error("Render failed: {0}")]
    RenderError(#[from] handlebars::RenderError),
    #[error("IO failed: {0}")]
    IoError(#[from] std::io::Error),
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn generate_docs(data: &RepositoryData, output : String) -> Result<(), DocsError> {
    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "assets/layouts")?;
    hb.register_templates_directory(".hbs", "assets/templates")?;
    
    render_static("index", data, &hb, &output)?;
    render_static("search", data, &hb, &output)?;

    copy_dir_all("assets/public", &output)?;


    Ok(())
}

fn render_static(name : &str, data: &RepositoryData, hb: &Handlebars, output : &str) -> Result<(), DocsError> {
    let context = &json!({
        "info": data.info
    });
    let rendered = hb.render(&name, context)?;
    std::fs::write(format!("{}/{}.html", output, name), rendered)?;
    Ok(())
}
