use crate::models::Named;
use serde::{Deserialize, Serialize};

// Path: content/{author}/{name}/asset.toml
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Asset {
    pub author: String,
    pub name: String,
    pub id: String,
    pub tags: Vec<String>,
    pub thumbnail_url: Option<String>,
    pub preview_urls: Option<Vec<String>>,
    pub current_version: Version,
    pub previous_versions: Vec<Version>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub sponsor: Option<String>,
    pub source: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Version {
    pub name: String,
    pub changes: String,
    pub download_url: String,
    pub sha256: String,
    pub blake3: Option<String>,
}

// Path: content/{author}/author.toml
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub links: Vec<String>,
}

impl Named for Asset {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Author {
    fn name(&self) -> &str {
        &self.name
    }
}
