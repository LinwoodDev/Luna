use std::{fs, io::Write, path::Path};

use handlebars::handlebars_helper;
use luna_api::models::{RepositoryData, asset::Version};
use luna_generator::{route::LunaRouter, template::handlebars::HandlebarsTemplateEngine};
use rust_embed::RustEmbed;
use serde_json::{Map, Value, json};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocsError {
    #[error("Template invalid: {0}")]
    Template(Box<dyn std::error::Error + Send + Sync>),
    #[error("Render {0} failed: {1}")]
    Render(String, Box<dyn std::error::Error + Send + Sync>),
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

fn build_engine() -> Result<HandlebarsTemplateEngine<'static>, DocsError> {
    let mut engine = HandlebarsTemplateEngine::new();
    handlebars_helper!(markdown_helper: |content : str| {
        markdown::to_html_with_options(content, &markdown::Options::gfm()).unwrap_or(content.to_string())
    });
    engine
        .registry()
        .register_helper("md", Box::new(markdown_helper));
    handlebars_helper!(inc: |x: i64| x + 1);
    handlebars_helper!(dec: |x: i64| x - 1);
    handlebars_helper!(sub: |a: i64, b: i64| a - b);
    handlebars_helper!(concat: |*args| {
        args.iter()
            .map(|v| v.as_str().unwrap_or(""))
            .collect::<Vec<&str>>()
            .join("")
    });
    // First check display Some<String> and then name
    handlebars_helper!(display: |named: Value| {
        let v = named
            .get("display_name")
            .or_else(|| named.get("name"))
            .unwrap_or(&named);
        match v {
            Value::String(s) => s.clone(),
            other            => other.to_string(),
        }
    });
    // slug helper for building category URLs
    handlebars_helper!(slug: |s: str| {
        slugify(s)
    });
    engine
        .registry()
        .register_helper("display", Box::new(display));
    engine
        .registry()
        .register_helper("concat", Box::new(concat));
    engine.registry().register_helper("inc", Box::new(inc));
    engine.registry().register_helper("dec", Box::new(dec));
    engine.registry().register_helper("sub", Box::new(sub));
    engine.registry().register_helper("slug", Box::new(slug));
    engine
        .registry()
        .register_embed_templates::<Templates>()
        .map_err(|e| DocsError::Template(Box::new(e)))?;
    Ok(engine)
}

// Simple slugify for route paths
fn slugify<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref().to_lowercase();
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
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
    let engine = build_engine()?;

    let add_route = |router: &mut LunaRouter,
                     path: &str,
                     template: &str,
                     context: &Value|
     -> Result<(), DocsError> {
        router
            .add_context_route(path, &engine, template, context)
            .map_err(|e| DocsError::Render(path.to_string(), e))
    };
    router.add_simple_route("index.json", data.to_index().unwrap());
    let mut base_context = Map::new();
    base_context.insert("info".to_string(), json!(data.info));
    base_context.insert("root".to_string(), json!(root));
    base_context.insert(
        "luna".to_string(),
        json!({ "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION") }),
    );

    // Build categories list and summaries
    let mut all_categories: Vec<String> = if !data.info.categories.is_empty() {
        data.info.categories.clone()
    } else {
        let mut set = std::collections::BTreeSet::new();
        for a in &data.assets {
            for c in &a.categories {
                set.insert(c.clone());
            }
        }
        set.into_iter().collect()
    };
    // Ensure stable order
    all_categories.sort();

    let mut categories_summaries: Vec<Map<String, Value>> = Vec::new();
    for name in &all_categories {
        let count = data
            .assets
            .iter()
            .filter(|a| a.categories.contains(name))
            .count();
        let mut m = Map::new();
        m.insert("name".to_string(), Value::String(name.clone()));
        m.insert("count".to_string(), json!(count));
        categories_summaries.push(m);
    }
    base_context.insert("categories".to_string(), json!(all_categories));

    let context = &Value::Object(base_context.clone());
    add_route(&mut router, "index.html", "templates/index.hbs", context)?;
    add_route(&mut router, "search.html", "templates/search.hbs", context)?;

    // Categories overview page
    {
        let mut ctx = base_context.clone();
        ctx.insert(
            "categories_summaries".to_string(),
            json!(categories_summaries),
        );
        let v = &Value::Object(ctx);
        add_route(&mut router, "categories.html", "templates/categories.hbs", v)?;
    }

    // Per-category pages (paginated)
    for cat in all_categories.iter() {
        let assets_for_cat: Vec<_> = data
            .assets
            .iter()
            .filter(|a| a.categories.contains(cat))
            .collect();
        let mut chunks = assets_for_cat.chunks(page_size).collect::<Vec<_>>();
        if chunks.is_empty() {
            chunks.push(&[]);
        }
        let page_count = chunks.len();
        for (i, chunk) in chunks.into_iter().enumerate() {
            let mut ctx = base_context.clone();
            let slug = slugify(cat);
            // pagination helpers and counts
            let assets_count = chunk.len();
            ctx.insert("category".to_string(), json!({ "name": cat }));
            ctx.insert("assets".to_string(), json!(chunk));
            ctx.insert("assets_count".to_string(), json!(assets_count));
            ctx.insert("current_page".to_string(), json!(i + 1));
            ctx.insert("last_page".to_string(), json!(page_count));
            let path = if i == 0 {
                format!("categories/{}.html", slug)
            } else {
                format!("categories/{}/{}.html", slug, i)
            };
            let v = &Value::Object(ctx);
            add_route(&mut router, &path, "templates/category.hbs", v)?;
        }
    }

    let asset_pages = data.assets.chunks(page_size);
    let page_count = asset_pages.len();
    for (i, assets) in asset_pages.enumerate() {
        let mut asset_context = base_context.clone();
        asset_context.insert("assets".to_string(), json!(assets));
        asset_context.insert("current_page".to_string(), json!(i + 1));
        asset_context.insert("last_page".to_string(), json!(page_count));
        let context = &Value::Object(asset_context);
        let path = if i == 0 {
            "assets.html".to_string()
        } else {
            format!("assets/{i}.html")
        };
        add_route(&mut router, &path, "templates/assets.hbs", context)?;
    }

    let author_pages = data.authors.chunks(page_size);
    let page_count = author_pages.len();
    for (i, authors) in author_pages.enumerate() {
        let mut asset_context = base_context.clone();
        asset_context.insert("authors".to_string(), json!(authors));
        asset_context.insert("current_page".to_string(), json!(i + 1));
        asset_context.insert("last_page".to_string(), json!(page_count));
        let context = &Value::Object(asset_context);
        let path = if i == 0 {
            "authors.html".to_string()
        } else {
            format!("authors/{i}.html")
        };
        add_route(&mut router, &path, "templates/authors.hbs", context)?;
    }

    for author in data.authors.iter() {
        let mut author_context = base_context.clone();
        author_context.insert("author".to_string(), json!(author));
        author_context.insert(
            "assets".to_string(),
            json!(
                data.assets
                    .iter()
                    .filter(|a| a.author == author.name)
                    .collect::<Vec<_>>()
            ),
        );
        let context = &Value::Object(author_context);
        add_route(
            &mut router,
            &format!("{}/index.html", author.name),
            "templates/author.hbs",
            context,
        )?;
    }

    for asset in data.assets.iter() {
        let mut asset_context = base_context.clone();
        asset_context.insert("asset".to_string(), json!(asset));
        let context = &Value::Object(asset_context);
        for page in ASSET_PAGES {
            add_route(
                &mut router,
                &format!("{}/{}/{}.html", asset.author, asset.name, page),
                &format!("templates/asset/{page}.hbs"),
                context,
            )?;
            let mut build_download_page =
                |path: &str, version: &Version| -> Result<(), DocsError> {
                    let mut version_context = base_context.clone();
                    version_context.insert("asset".to_string(), json!(asset));
                    version_context.insert("version".to_string(), json!(version));
                    let v_ctx = &Value::Object(version_context);
                    add_route(&mut router, path, "templates/asset/download.hbs", v_ctx)?;
                    Ok(())
                };
            build_download_page(
                &format!("{}/{}/download.html", asset.author, asset.name),
                &asset.current_version,
            )?;
            for version in
                std::iter::once(&asset.current_version).chain(asset.previous_versions.iter())
            {
                build_download_page(
                    &format!(
                        "{}/{}/download/{}.html",
                        asset.author, asset.name, version.name
                    ),
                    version,
                )?;
            }
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
