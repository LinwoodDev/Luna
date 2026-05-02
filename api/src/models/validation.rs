use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

use super::RepositoryData;
use super::asset::{Asset, Version};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
}

impl ValidationError {
    fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    pub fn new(errors: Vec<ValidationError>) -> Self {
        Self { errors }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }
}

impl Display for ValidationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} validation error(s)", self.errors.len())?;
        for error in &self.errors {
            writeln!(f, "- {}: {}", error.path, error.message)?;
        }
        Ok(())
    }
}

impl Error for ValidationErrors {}

pub fn validate_repository(data: &RepositoryData) -> Result<(), ValidationErrors> {
    let mut errors = Vec::new();

    if data.file_version != 1 {
        errors.push(ValidationError::new(
            "file_version",
            format!("expected 1, found {}", data.file_version),
        ));
    }
    require_name("name", &data.info.name, &mut errors);

    let mut author_names = HashSet::new();
    for (index, author) in data.authors.iter().enumerate() {
        let path = format!("authors[{index}]");
        require_name(format!("{path}.name"), &author.name, &mut errors);
        if !author.name.is_empty() && !author_names.insert(author.name.as_str()) {
            errors.push(ValidationError::new(
                format!("{path}.name"),
                format!("duplicate author {:?}", author.name),
            ));
        }
        validate_optional_urlish(
            format!("{path}.avatar_url"),
            author.avatar_url.as_deref(),
            &mut errors,
        );
        for (link_index, link) in author.links.iter().enumerate() {
            validate_urlish(format!("{path}.links[{link_index}]"), link, &mut errors);
        }
    }

    let mut asset_ids = HashSet::new();
    let mut asset_names = HashSet::new();
    for (index, asset) in data.assets.iter().enumerate() {
        let path = format!("assets[{index}]");
        validate_asset(asset, &path, &author_names, &mut errors);
        if !asset.id.is_empty() && !asset_ids.insert(asset.id.as_str()) {
            errors.push(ValidationError::new(
                format!("{path}.id"),
                format!("duplicate asset id {:?}", asset.id),
            ));
        }
        if !asset.author.is_empty()
            && !asset.name.is_empty()
            && !asset_names.insert((asset.author.as_str(), asset.name.as_str()))
        {
            errors.push(ValidationError::new(
                format!("{path}.name"),
                format!("duplicate asset {:?}/{}", asset.author, asset.name),
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ValidationErrors::new(errors))
    }
}

fn validate_asset(
    asset: &Asset,
    path: &str,
    author_names: &HashSet<&str>,
    errors: &mut Vec<ValidationError>,
) {
    require_name(format!("{path}.author"), &asset.author, errors);
    require_name(format!("{path}.name"), &asset.name, errors);
    require_name(format!("{path}.id"), &asset.id, errors);

    if !asset.author.is_empty() && !author_names.contains(asset.author.as_str()) {
        errors.push(ValidationError::new(
            format!("{path}.author"),
            format!("unknown author {:?}", asset.author),
        ));
    }

    let expected_id = format!("{}/{}", asset.author, asset.name);
    if asset.id != expected_id {
        errors.push(ValidationError::new(
            format!("{path}.id"),
            format!("expected {:?}", expected_id),
        ));
    }

    validate_optional_urlish(
        format!("{path}.thumbnail_url"),
        asset.thumbnail_url.as_deref(),
        errors,
    );
    if let Some(urls) = &asset.preview_urls {
        for (index, url) in urls.iter().enumerate() {
            validate_urlish(format!("{path}.preview_urls[{index}]"), url, errors);
        }
    }
    validate_optional_urlish(format!("{path}.sponsor"), asset.sponsor.as_deref(), errors);
    validate_optional_urlish(format!("{path}.source"), asset.source.as_deref(), errors);
    validate_optional_urlish(format!("{path}.website"), asset.website.as_deref(), errors);
    for (index, category) in asset.categories.iter().enumerate() {
        require_name(format!("{path}.categories[{index}]"), category, errors);
    }

    validate_version(
        &asset.current_version,
        &format!("{path}.current_version"),
        errors,
    );
    for (index, version) in asset.previous_versions.iter().enumerate() {
        validate_version(
            version,
            &format!("{path}.previous_versions[{index}]"),
            errors,
        );
    }
}

fn validate_version(version: &Version, path: &str, errors: &mut Vec<ValidationError>) {
    require_name(format!("{path}.name"), &version.name, errors);
    validate_urlish(
        format!("{path}.download_url"),
        &version.download_url,
        errors,
    );
    validate_hash(format!("{path}.sha256"), &version.sha256, 64, errors);
    if let Some(blake3) = &version.blake3 {
        validate_hash(format!("{path}.blake3"), blake3, 64, errors);
    }
}

fn require_name(path: impl Into<String>, value: &str, errors: &mut Vec<ValidationError>) {
    if value.trim().is_empty() {
        errors.push(ValidationError::new(path, "must not be empty"));
    }
}

fn validate_optional_urlish(
    path: impl Into<String>,
    value: Option<&str>,
    errors: &mut Vec<ValidationError>,
) {
    let Some(value) = value else {
        return;
    };
    validate_urlish(path, value, errors);
}

fn validate_urlish(path: impl Into<String>, value: &str, errors: &mut Vec<ValidationError>) {
    let valid = !value.trim().is_empty()
        && !value.chars().any(char::is_whitespace)
        && (value.starts_with("https://")
            || value.starts_with("http://")
            || value.starts_with('/')
            || value.starts_with("./")
            || value.starts_with("../"));
    if !valid {
        errors.push(ValidationError::new(
            path,
            "must be an http(s), absolute, or relative path without whitespace",
        ));
    }
}

fn validate_hash(
    path: impl Into<String>,
    value: &str,
    expected_len: usize,
    errors: &mut Vec<ValidationError>,
) {
    let valid = value.len() == expected_len && value.chars().all(|c| c.is_ascii_hexdigit());
    if !valid {
        errors.push(ValidationError::new(
            path,
            format!("must be a {expected_len}-character hexadecimal digest"),
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::models::asset::{Asset, Author, Version};
    use crate::models::{RepositoryData, RepositoryInfo};

    fn valid_version() -> Version {
        Version {
            name: "1.0.0".to_string(),
            changes: "Initial release".to_string(),
            download_url: "https://example.com/pack.zip".to_string(),
            sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
            ..Default::default()
        }
    }

    fn valid_repository() -> RepositoryData {
        RepositoryData {
            info: RepositoryInfo {
                name: "Example".to_string(),
                ..Default::default()
            },
            authors: vec![Author {
                name: "linwood".to_string(),
                links: vec!["https://linwood.dev".to_string()],
                ..Default::default()
            }],
            assets: vec![Asset {
                author: "linwood".to_string(),
                name: "starter-pack".to_string(),
                id: "linwood/starter-pack".to_string(),
                current_version: valid_version(),
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    #[test]
    fn accepts_valid_repository() {
        assert!(valid_repository().validate().is_ok());
    }

    #[test]
    fn rejects_drifting_asset_id() {
        let mut repository = valid_repository();
        repository.assets[0].id = "wrong/id".to_string();

        let errors = repository.validate().unwrap_err();

        assert!(
            errors
                .errors()
                .iter()
                .any(|error| error.path == "assets[0].id")
        );
    }

    #[test]
    fn rejects_unknown_author_and_bad_hash() {
        let mut repository = valid_repository();
        repository.assets[0].author = "missing".to_string();
        repository.assets[0].id = "missing/starter-pack".to_string();
        repository.assets[0].current_version.sha256 = "nope".to_string();

        let errors = repository.validate().unwrap_err();

        assert!(
            errors
                .errors()
                .iter()
                .any(|error| error.path == "assets[0].author")
        );
        assert!(
            errors
                .errors()
                .iter()
                .any(|error| error.path == "assets[0].current_version.sha256")
        );
    }
}
