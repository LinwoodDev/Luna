use crate::directory::{ModelDirectory, ModelError, RepositoryDirectory};
use luna_api::models::RepositoryData;
use thiserror::Error;

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
                if !model
                    .categories
                    .iter()
                    .any(|c| data.info.exclude_categories.contains(c))
                {
                    data.assets.push(model);
                }
            }
        }
        Ok(data)
    }
}
