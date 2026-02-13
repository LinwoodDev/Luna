pub mod asset;

use std::collections::HashMap;

use asset::*;
use serde::{Deserialize, Serialize};

const FILE_VERSION: u8 = 1;

pub trait Named {
    fn name(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryData {
    pub assets: Vec<Asset>,
    pub authors: Vec<Author>,
    #[serde(flatten)]
    pub info: RepositoryInfo,
    pub file_version: u8,
}

impl Default for RepositoryData {
    fn default() -> Self {
        Self {
            assets: Default::default(),
            authors: Default::default(),
            info: Default::default(),
            file_version: FILE_VERSION,
        }
    }
}

impl Named for RepositoryData {
    fn name(&self) -> &str {
        &self.info.name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RepositoryInfo {
    pub name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub exclude_categories: Vec<String>,
    pub imprint: Option<String>,
    pub privacy: Option<String>,
    #[serde(default)]
    pub links: HashMap<String, String>,
    #[serde(default)]
    pub navbar_links: HashMap<String, String>,
}

impl Named for RepositoryInfo {
    fn name(&self) -> &str {
        &self.name
    }
}

impl RepositoryData {
    pub fn new() -> RepositoryData {
        Self::default()
    }
    pub fn from_index(data: &str) -> Result<RepositoryData, serde_json::Error> {
        serde_json::from_str(data)
    }

    pub fn to_index(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}
