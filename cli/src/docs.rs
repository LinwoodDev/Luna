use std::{fs, path::Path, io::Write};

use handlebars::{Handlebars, TemplateError};
use luna_api::models::RepositoryData;
use serde_json::{json, Value};
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

const ASSET_PAGES: [&str; 2] = ["index", "changes"];

pub fn generate_docs(data: &RepositoryData, output : String) -> Result<(), DocsError> {
    let mut hb = Handlebars::new();
    hb.register_embed_templates::<Templates>()?;
    
    render_static("index", data, &hb, &output)?;
    render_static("search", data, &hb, &output)?;


    for author in data.authors.iter() {
        fs::create_dir_all(format!("{}/{}", output, author.name))?;
    }
    for asset in data.assets.iter() {
        let context: &Value = &json!({
            "asset": asset,
            "info": data.info
        });
        fs::create_dir_all(format!("{}/{}/{}", output, &asset.author, &asset.name))?;
        for page in ASSET_PAGES {
            render_dynamic(&format!("asset/{}", &page), &format!("{}/{}/{}", &asset.author, &asset.name, &page), &hb, &output, context)?;
        }
    }

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

fn copy_public(output : &str) -> Result<(), DocsError> {
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
    let context: &Value = &json!({
        "info": data.info
    });
    render_dynamic(name, name, hb, output, context)
}

fn render_dynamic(name : &str, output_name : &str, hb: &Handlebars, output : &str, context: &Value) -> Result<(), DocsError> {
    let rendered = hb.render(&format!("templates/{}.hbs",name), &context)?;
    fs::write(format!("{}/{}.html", output, output_name), rendered)?;
    println!("Rendered {} at {}/{}.html", name, output, output_name);
    Ok(())

}
