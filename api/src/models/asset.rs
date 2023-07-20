use serde::{Deserialize, Serialize};

// Path: assets/{author}/{name}/asset.toml
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub author : String,
    pub name : String,
    pub id : String,
    pub tags : Vec<String>,
    pub thumbnail_url : String,
    pub preview_urls : Vec<String>,
    pub current_version : Version,
    pub previous_versions : Vec<Version>,
    pub summary : Option<String>,
    pub description : Option<String>,
    pub sponsor : Option<String>,
    pub source : Option<String>,
    pub website : Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
    pub name : String,
    pub changes : String,
    pub download_url : String,
    pub sha256 : String,
    pub blake3 : Option<String>,
}

// Path: assets/{author}/author.toml
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Author {
    pub name : String,
    pub email : String,
    pub links : Vec<String>,
}
