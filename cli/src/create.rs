use std::{fs::File, io::Write};

use anyhow::Context;
use luna_api::models::{RepositoryData, RepositoryInfo, asset::Author};

use crate::{CreateAuthorArgs, CreateCommands, CreateRepositoryArgs};

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


pub fn create(create_command: &CreateCommands) -> anyhow::Result<()> {
    match create_command {
        CreateCommands::Repository(args) => create_repository(args),
        CreateCommands::Author(args) => create_author(args),
    }
}
