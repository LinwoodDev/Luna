use std::collections::HashMap;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::io::Error as IoError;
use std::path::{Component, Path, PathBuf};

use crate::template::TemplateEngine;

#[derive(Debug, Default, Clone)]
pub struct LunaRouter {
    routes: Vec<Route>,
}

#[derive(Clone)]
pub struct Route {
    path: String,
    data: String,
}

impl LunaRouter {
    pub fn new() -> Self {
        LunaRouter { routes: Vec::new() }
    }

    pub fn add_context_route<T, C>(
        &mut self,
        path: &str,
        engine: &T,
        template: &str,
        context: &C,
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        T: TemplateEngine,
        C: serde::Serialize,
    {
        let data = engine.render(template, context)?;
        self.add_simple_route(path, data);
        Ok(())
    }

    pub fn add_simple_route(&mut self, path: &str, data: String) {
        self.routes.push(Route::new(path, data))
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    pub fn render(&self, path: &str) -> Option<String> {
        self.routes
            .iter()
            .find(|r| r.path == path)
            .map(|r| r.data.clone())
    }

    pub fn render_all(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for route in &self.routes {
            map.insert(route.path.clone(), route.data.clone());
        }
        map
    }

    pub fn generate<P: AsRef<Path>>(&self, directory: P) -> Result<(), IoError> {
        let directory = directory.as_ref();
        for (path, content) in self.render_all() {
            let output = output_path(directory, &path);
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    IoError::new(
                        e.kind(),
                        format!("Failed to create directory for file {path}: {e}"),
                    )
                })?;
            }
            std::fs::write(output, content).map_err(|e| {
                IoError::new(e.kind(), format!("Failed to write to file {path}: {e}"))
            })?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route").field("path", &self.path).finish()
    }
}

impl Route {
    pub fn new(path: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            path: normalize_route_path(&path.into()),
            data: data.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn content(&self) -> &str {
        &self.data
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Route {}

impl Hash for Route {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

fn output_path(root: &Path, route_path: &str) -> PathBuf {
    let mut output = root.to_path_buf();
    for component in Path::new(route_path).components() {
        match component {
            Component::Normal(part) => output.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {}
        }
    }
    output
}

fn normalize_route_path(path: &str) -> String {
    let mut normalized = Vec::new();
    for component in Path::new(path).components() {
        match component {
            Component::Normal(part) => normalized.push(part.to_string_lossy().to_string()),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {}
        }
    }
    normalized.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_routes_for_output() {
        let route = Route::new("/../docs/./index.html", "hello");

        assert_eq!(route.path(), "docs/index.html");
        assert_eq!(route.content(), "hello");
    }

    #[test]
    fn overwrites_duplicate_paths_when_rendering_all() {
        let mut router = LunaRouter::new();
        router.add_simple_route("index.html", "first".to_string());
        router.add_simple_route("index.html", "second".to_string());

        assert_eq!(
            router.render_all().get("index.html"),
            Some(&"second".to_string())
        );
    }
}
