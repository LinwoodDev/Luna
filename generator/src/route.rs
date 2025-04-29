use std::collections::HashMap;
use std::io::Error;
use std::iter::Map;
use std::sync::Arc;
use std::hash::{Hash, Hasher};

#[derive(Debug, Default, Clone)]
pub struct LunaRouter {
    routes: Vec<Route>,
}

#[derive(Clone)]
pub struct Route {
    path: String,
    renderer: Arc<dyn Fn() -> String + Send + Sync>,
}

impl LunaRouter {
    pub fn new() -> Self {
        LunaRouter { routes: Vec::new() }
    }

    pub fn add_route<F>(&mut self, path: &str, renderer: F) -> &mut Self
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: path.to_string(),
            renderer: Arc::new(renderer),
        });
        self
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    pub fn render(&self, path: &str) -> Option<String> {
        self.routes.iter().find(|r| r.path == path).map(|r| (r.renderer)())
    }

    pub fn render_all(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for route in &self.routes {
            map.insert(route.path.clone(), (route.renderer)());
        }
        map
    }

    pub fn generate(&self, directory: &str) -> Result<(), Error> {
        for route in &self.routes {
            let path = format!("{}/{}", directory, route.path);
            let content = (route.renderer)();
            std::fs::write(path, content).map_err(|e| {
                Error::new(
                    e.kind(),
                    format!("Failed to write to file {}: {}", route.path, e),
                )
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
    pub fn new<F>(path: impl Into<String>, renderer: F) -> Self
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        Route { path: path.into(), renderer: Arc::new(renderer) }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn render(&self) -> String {
        (self.renderer)()
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