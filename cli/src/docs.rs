use std::{fs, io::Write, path::Path};

use luna_api::models::RepositoryData;
use luna_generator::{route::LunaRouter, template::handlebars::HandlebarsTemplateEngine};
use rust_embed::RustEmbed;
use serde_json::{Value, json, Map};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocsError {
    #[error("Template invalid: {0}")]
    Template(Box<dyn std::error::Error>),
    #[error("Render failed: {0}")]
    Render(#[from] handlebars::RenderError),
    #[error("IO failed: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(RustEmbed)]
#[folder = "assets"]
#[exclude = "public/*"]
struct Templates;
#[derive(RustEmbed)]
#[folder = "assets/public"]
struct Public;

const ASSET_PAGES: [&str; 2] = ["index", "changes"];

fn wrap_template_error<T>(e: Result<T, Box<dyn std::error::Error>>) -> Result<T, DocsError> {
    e.map_err(DocsError::Template)
}

pub fn generate_docs(
    data: &RepositoryData,
    output: String,
    page_size: usize,
) -> Result<(), DocsError> {
    let output_path = Path::new(&output);
    if output_path.exists() {
        fs::remove_dir_all(output_path)?;
    }
    let _ = page_size;
    let mut router = LunaRouter::new();
    let root = "/".to_string();
    let mut engine = HandlebarsTemplateEngine::new();
    engine
        .registry()
        .register_embed_templates::<Templates>()
        .map_err(|e| DocsError::Template(Box::new(e)))?;
    router.add_simple_route("index.json", serde_json::to_string(data).unwrap());
    let mut base_context = Map::new();
    base_context.insert("info".to_string(), json!(data.info));
    base_context.insert("root".to_string(), json!(root));
    base_context.insert("luna".to_string(), json!({ "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION") }));
    let context = &Value::Object(base_context.clone());
    wrap_template_error(router.add_context_route(
        "index.html",
        &engine,
        "templates/index.hbs",
        context,
    ))?;
    wrap_template_error(router.add_context_route(
        "search.html",
        &engine,
        "templates/search.hbs",
        context,
    ))?;

    for asset in data.assets.iter() {
        let mut asset_context = base_context.clone();
        asset_context.insert("asset".to_string(), json!(asset));
        let context = &Value::Object(asset_context);
        for page in ASSET_PAGES {
            wrap_template_error(router.add_context_route(
                &format!("{}/{}/asset/{}.html", asset.author, asset.name, page),
                &engine,
                &format!("templates/asset/{page}.hbs"),
                context,
            ))?;
        }
    }

    copy_public(&output)?;
    router.generate(&output)?;

    Ok(())
}

fn copy_public(output: &str) -> Result<(), DocsError> {
    for file in Public::iter() {
        let path = file.as_ref();
        let content = Public::get(path).unwrap();
        let path = format!("{output}/{path}");
        let path = Path::new(&path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        file.write_all(&content.data)?;
    }
    Ok(())
}
