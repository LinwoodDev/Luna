use std::collections::HashMap;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::io::Error as IoError;

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
    ) -> Result<(), Box<dyn Error>>
    where
        T: TemplateEngine,
        C: serde::Serialize,
    {
        let data = engine.render(template, context)?;
        self.add_simple_route(path, data);
        Ok(())
    }

    pub fn add_simple_route(&mut self, path: &str, data: String)  {
        self.routes.push(Route {
            path: path.to_string(),
            data,
        })
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

    pub fn generate(&self, directory: &str) -> Result<(), IoError> {
        for (path, content) in self.render_all() {
            let output = format!("{directory}/{path}");
            std::fs::create_dir_all(std::path::Path::new(&output).parent().unwrap()).map_err(
                |e| {
                    IoError::new(
                        e.kind(),
                        format!("Failed to create directory for file {path}: {e}"),
                    )
                },
            )?;
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
    pub fn path(&self) -> &str {
        &self.path
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
