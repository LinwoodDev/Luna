use thiserror::Error;
use api::models::RepositoryData;
use crate::directory::{ModelDirectory, ModelError, RepositoryDirectory};

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Model generation failed: {0}")]
    ModelError(#[from] ModelError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl RepositoryDirectory {
    pub fn generate_index(&self) -> Result<RepositoryData, GeneratorError> {
        let mut data = RepositoryData::new();
        data.info = self.model()?;
        let authors = self.authors()?;
        for author in authors {
            let directory = self.author(&author);
            let model = directory.model()?;
            data.authors.push(model);
            for asset in directory.assets()? {
                let directory = directory.asset(&asset);
                let model = directory.model()?;
                data.assets.push(model);
            }
        }
        Ok(data)
    }
}
