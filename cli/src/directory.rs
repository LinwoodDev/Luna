use std::io;
use std::path::PathBuf;
use serde::de::DeserializeOwned;
use thiserror::Error;
use toml::de;
use luna_api::models::{asset::{Asset, Author}, Named, RepositoryInfo};
use crate::directory::ModelError::ModelFileNotFound;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    TomlError(#[from] de::Error),
    #[error("Model file not found (expected {0:?})")]
    ModelFileNotFound(Option<PathBuf>),
    #[error("Model name not valid (expected {expected:?}, found {found:?})")]
    NotValidName { expected: Option<String>, found: String },
}

pub struct RepositoryDirectory(pub PathBuf);

pub trait ModelDirectory<T>
    where T: DeserializeOwned + Named + Default {
    fn data_path(&self) -> PathBuf;
    fn is_valid(&self) -> bool {
        self.data_path().is_dir()
    }
    fn file_path(&self) -> PathBuf {
        self.data_path().join("config.toml")
    }
    fn content_path(&self) -> PathBuf {
        self.data_path().join("content")
    }
    fn name(&self) -> Option<String>;
    fn model(&self) -> Result<T, ModelError> {
        let file = self.file_path();
        if !self.is_valid() {
            return Err(ModelFileNotFound(Some(file)));
        }

        let data = std::fs::read_to_string(file)?;
        let model: T = toml::from_str(&data)?;
        if self.name().map(|name| name != model.name()).unwrap_or(false) {
            return Err(ModelError::NotValidName {
                expected: self.name(),
                found: model.name().to_owned(),
            });
        }

        Ok(model)
    }
    fn valid_model(&self) -> bool {
        self.is_valid() && self.model().is_ok()
    }
}

impl RepositoryDirectory {
    pub fn new(path: Option<&str>) -> RepositoryDirectory {
        let path = match path {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("."),
        };

        RepositoryDirectory(path)
    }

    pub fn authors(&self) -> Result<Vec<String>, io::Error> {
        let mut authors = Vec::new();
        let content = self.content_path();
        if !content.exists() {
            return Ok(authors);
        }
        for entry in content.read_dir()? {
            let Ok(entry) = entry else {
                continue;
            };
            if !entry.path().is_dir() {
                continue;
            }
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

            if path.is_dir() {
                authors.push(file_name);
            }
        }

        Ok(authors)
    }

    pub fn author(&self, name: &str) -> AuthorDirectory {
        AuthorDirectory(self, name.to_owned())
    }
}

impl ModelDirectory<RepositoryInfo> for RepositoryDirectory {
    fn data_path(&self) -> PathBuf {
        self.0.to_path_buf()
    }

    fn name(&self) -> Option<String> {
        None
    }
}

pub struct AuthorDirectory<'a> (&'a RepositoryDirectory, String);

impl AuthorDirectory<'_> {
    pub fn assets(&self) -> Result<Vec<String>, io::Error> {
        let mut assets = Vec::new();
        let path = self.assets_path();
        if !path.exists() {
            return Ok(assets);
        }
        for entry in path.read_dir()? {
            let Ok(entry) = entry else {
                continue;
            };
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

            if path.is_dir() {
                assets.push(file_name);
            }
        }

        Ok(assets)
    }

    pub fn assets_path(&self) -> PathBuf {
        self.data_path().join("assets")
    }

    pub fn asset(&self, name: &str) -> AssetDirectory {
        AssetDirectory(self, name.to_owned())
    }
}

impl ModelDirectory<Author> for AuthorDirectory<'_> {
    fn data_path(&self) -> PathBuf {
        self.0.content_path().join(&self.1)
    }

    fn name(&self) -> Option<String> {
        Some(self.1.clone())
    }
}

pub struct AssetDirectory<'a> (&'a AuthorDirectory<'a>, String);

impl ModelDirectory<Asset> for AssetDirectory<'_> {
    fn data_path(&self) -> PathBuf {
        self.0.assets_path().join(&self.1)
    }

    fn name(&self) -> Option<String> {
        Some(self.1.clone())
    }
}
