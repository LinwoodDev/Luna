pub mod asset;

use asset::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryData {
    pub name : String,
    pub description : String,
    pub tags : Vec<String>,
    pub assets : Vec<Asset>,
    pub authors : Vec<Author>,
}

impl RepositoryData {
    pub fn from_index(data : &str) -> Result<RepositoryData, serde_json::Error> {
        serde_json::from_str(data)
    }
}
