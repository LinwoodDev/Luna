use std::io;
use std::path::PathBuf;
use api::models::asset::Author;

struct RepositoryDirectory(pub PathBuf);

impl RepositoryDirectory {
    pub fn new(path: Option<&str>) -> RepositoryDirectory {
        let path = match path {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("."),
        };

        RepositoryDirectory(path)
    }

    fn data_path(&self) -> PathBuf {
        self.0.join("data")
    }

    pub fn authors(&self) -> Result<Vec<String>, io::Error> {
        let mut assets = Vec::new();
        for entry in self.data_path().read_dir()? {
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

    pub fn author(&self, name: &str) -> Option<AuthorDirectory> {
        Some(AuthorDirectory(self, name.to_owned()))
    }
}

struct AuthorDirectory<'a> (&'a RepositoryDirectory, String);

impl AuthorDirectory<'_> {
    pub fn data_path(&self) -> PathBuf {
        self.0.data_path().join(&self.1)
    }

    pub fn is_valid(&self) -> bool {
        self.data_path().is_dir()
    }

    pub fn file_path(&self) -> PathBuf {
        self.data_path().join("author.json")
    }

    pub fn model(&self) -> Option<Author> {
        if !self.is_valid() {
            return None;
        }

        let data = std::fs::read_to_string(self.file_path()).ok()?;
        let model: Author = toml::from_str(&data).ok()?;
        if model.name != self.1 {
            return None;
        }

        Some(model)
    }

    pub fn valid_model(&self) -> bool {
        self.file_path().is_file() && self.model().is_some()
    }

    pub fn assets(&self) -> Result<Vec<String>, io::Error> {
        let mut assets = Vec::new();
        for entry in self.data_path().read_dir()? {
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

    pub fn asset(&self, name: &str) -> Option<AssetDirectory> {
        let path = self.data_path().join(name);

        if path.is_dir() {
            Some(AssetDirectory(self, name.to_owned()))
        } else {
            None
        }
    }
}

struct AssetDirectory<'a> (&'a AuthorDirectory<'a>, String);

impl AssetDirectory<'_> {}
