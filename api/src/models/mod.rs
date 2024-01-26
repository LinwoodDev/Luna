pub mod asset;

use asset::*;
use serde::{Deserialize, Serialize};

const FILE_VERSION: u8 = 0;

pub trait Named {
    fn name(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RepositoryData {
    pub assets: Vec<Asset>,
    pub authors: Vec<Author>,
    #[serde(flatten)]
    pub info: RepositoryInfo,
    pub file_version: u8,
}

impl Named for RepositoryData {
    fn name(&self) -> &str {
        &self.info.name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RepositoryInfo {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl Named for RepositoryInfo {
    fn name(&self) -> &str {
        &self.name
    }
}

impl RepositoryData {
    pub fn new() -> RepositoryData {
        RepositoryData {
            file_version: FILE_VERSION,
            ..Default::default()
        }
    }
    pub fn from_index(data: &str) -> Result<RepositoryData, serde_json::Error> {
        serde_json::from_str(data)
    }

    pub fn to_index(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}
