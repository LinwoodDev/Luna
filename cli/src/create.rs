use std::{fs::File, io::Write};

use anyhow::Context;
use luna_api::models::{RepositoryData, RepositoryInfo};

use crate::{CreateCommands, CreateRepositoryArgs};

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
    Ok(())
}

pub fn create(create_command: &CreateCommands) -> anyhow::Result<()> {
    match create_command {
        CreateCommands::Repository(args) => create_repository(args),
    }
}
