use std::{fs, path::Path, io::{self, Write}};

use handlebars::{Handlebars, TemplateError, Context};
use luna_api::models::RepositoryData;
use serde_json::json;
use thiserror::Error;
use rust_embed::RustEmbed;

#[derive(Error, Debug)]
pub enum DocsError {
    #[error("Template invalid: {0}")]
    TemplateError(#[from] TemplateError),
    #[error("Render failed: {0}")]
    RenderError(#[from] handlebars::RenderError),
    #[error("IO failed: {0}")]
    IoError(#[from] std::io::Error),
}  

#[derive(RustEmbed)]
#[folder = "assets"]
#[exclude = "public/*"]
struct Templates;
#[derive(RustEmbed)]
#[folder = "assets/public"]
struct Public;

pub fn generate_docs(data: &RepositoryData, output : String) -> Result<(), DocsError> {
    let mut hb = Handlebars::new();
    hb.register_embed_templates::<Templates>()?;
    
    render_static("index", data, &hb, &output)?;
    render_static("search", data, &hb, &output)?;

    for file in Public::iter() {
        let path = file.as_ref();
        let content = Public::get(path).unwrap();
        let path = format!("{}/{}", output, path);
        let path = Path::new(&path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        file.write_all(&content.data)?;
    }

    Ok(())
}

fn render_static(name : &str, data: &RepositoryData, hb: &Handlebars, output : &str) -> Result<(), DocsError> {
    let context = &json!({
        "info": data.info
    });
    let rendered = hb.render(&format!("templates/{}.hbs",name), context)?;
    std::fs::write(format!("{}/{}.html", output, name), rendered)?;
    println!("Rendered {} at {}/{}.html", name, output, name);
    Ok(())
}
