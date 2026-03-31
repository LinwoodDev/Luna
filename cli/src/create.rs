use std::{fs::File, io::Write};

use anyhow::Context;
use luna_api::models::{RepositoryData, RepositoryInfo, asset::{Asset, Author}};

use crate::{CreateAssetArgs, CreateAuthorArgs, CreateCommands, CreateRepositoryArgs};

pub fn create_repository(args: &CreateRepositoryArgs) -> anyhow::Result<()> {
    let repository_path = std::path::Path::new(&args.path);
    let directory_name = repository_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("my-repository");
    let repository = RepositoryData {
        info: RepositoryInfo {
            name: directory_name.to_string(),
            description: args.description.clone(),
            ..Default::default()
        },
        ..Default::default()
    };
    let toml = toml::to_string(&repository)?;
        std::fs::create_dir_all(repository_path)
            .with_context(|| format!("Failed to create parent directories for {}", args.path))?;
    let mut file = File::create(repository_path.join("config.toml")).with_context(|| {
        format!(
            "Failed to create repository configuration file at {}",
            args.path
        )
    })?;
    file.write_all(toml.as_bytes())?;
    println!(
        "Repository configuration file created at {}",
        repository_path.display()
    );
    Ok(())
}


pub fn create_author(args: &CreateAuthorArgs) -> anyhow::Result<()> {
    let author = Author {
        name: args.name.clone(),
        display_name: args.display_name.clone(),
        avatar_url: args.avatar_url.clone(),
        description: args.description.clone(),
        email: args.email.clone(),
        links: args.links.clone(),
    };
    let toml = toml::to_string(&author)?;
    let author_path = std::path::Path::new(&args.path).join("content").join(&args.name);
    std::fs::create_dir_all(&author_path).with_context(|| {
        format!(
            "Failed to create directories for author at {}",
            author_path.display()
        )
    })?;
    let file_path = author_path.join("author.toml");
    if file_path.exists() && !args.force {
        return Err(anyhow::anyhow!(
            "Author file already exists at {}. Use --force to overwrite.",
            file_path.display()
        ));
    }
    let mut file = File::create(author_path.join("author.toml")).with_context(|| {
        format!(
            "Failed to create author file at {}",
            author_path.display()
        )
    })?;
    file.write_all(toml.as_bytes())?;
    println!(
        "Author file created at {}",
        author_path.display()
    );

    Ok(())
}

pub fn create_asset(args: &CreateAssetArgs) -> anyhow::Result<()> {
    let asset = Asset {
        author: args.author.clone(),
        name: args.name.clone(),
        id: format!("{}/{}", args.author, args.name),
        description: args.description.clone(),
        ..Default::default()
    };
    let toml = toml::to_string(&asset)?;
    let author_path = std::path::Path::new(&args.path).join("content").join(&args.author);
    if !author_path.exists() {
        return Err(anyhow::anyhow!(
            "Author directory does not exist at {}",
            author_path.display()
        ));
    }
    let asset_path = author_path.join("assets").join(&args.name);
    std::fs::create_dir_all(&asset_path).with_context(|| {
        format!(
            "Failed to create directories for asset at {}",
            asset_path.display()
        )
    })?;
    let file_path = asset_path.join("asset.toml");
    if file_path.exists() && !args.force {
        return Err(anyhow::anyhow!(
            "Asset file already exists at {}. Use --force to overwrite.",
            file_path.display()
        ));
    }
    let mut file = File::create(file_path).with_context(|| {
        format!(
            "Failed to create asset file at {}",
            asset_path.display()
        )
    })?;
    file.write_all(toml.as_bytes())?;
    println!(
        "Asset file created at {}",
        asset_path.display()
    );
    Ok(())
}


pub fn create(create_command: &CreateCommands) -> anyhow::Result<()> {
    match create_command {
        CreateCommands::Repository(args) => create_repository(args),
        CreateCommands::Author(args) => create_author(args),
        CreateCommands::Asset(args) => create_asset(args),
    }
}
