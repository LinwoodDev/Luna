pub mod asset;

use asset::*;
use serde::{Deserialize, Serialize};

pub trait Named {
    fn name(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RepositoryData {
    pub assets : Vec<Asset>,
    pub authors : Vec<Author>,
    #[serde(flatten)]
    pub info : RepositoryInfo,
}

impl Named for RepositoryData {
    fn name(&self) -> &str {
        &self.info.name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RepositoryInfo {
    pub name : String,
    pub description : String,
    pub tags : Vec<String>,
}

impl Named for RepositoryInfo {
    fn name(&self) -> &str {
        &self.name
    }
}


impl RepositoryData {
    pub fn new() -> RepositoryData {
        RepositoryData {
            ..Default::default()
        }
    }
    pub fn from_index(data : &str) -> Result<RepositoryData, serde_json::Error> {
        serde_json::from_str(data)
    }

    pub fn to_index(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}
