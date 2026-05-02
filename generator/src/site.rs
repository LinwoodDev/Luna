use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

use serde::Serialize;

use crate::route::LunaRouter;
use crate::template::TemplateEngine;

#[derive(Debug)]
pub enum StaticSiteError {
    Render(Box<dyn Error + Send + Sync>),
    Io(std::io::Error),
}

impl Display for StaticSiteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StaticSiteError::Render(error) => write!(f, "render failed: {error}"),
            StaticSiteError::Io(error) => write!(f, "io failed: {error}"),
        }
    }
}

impl Error for StaticSiteError {}

impl From<std::io::Error> for StaticSiteError {
    fn from(value: std::io::Error) -> Self {
        StaticSiteError::Io(value)
    }
}

#[derive(Debug, Default, Clone)]
pub struct StaticSite {
    router: LunaRouter,
}

impl StaticSite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn router(&self) -> &LunaRouter {
        &self.router
    }

    pub fn router_mut(&mut self) -> &mut LunaRouter {
        &mut self.router
    }

    pub fn add_page(&mut self, path: impl AsRef<str>, content: impl Into<String>) -> &mut Self {
        self.router.add_simple_route(path.as_ref(), content.into());
        self
    }

    pub fn add_template_page<T, C>(
        &mut self,
        path: impl AsRef<str>,
        engine: &T,
        template: &str,
        context: &C,
    ) -> Result<&mut Self, StaticSiteError>
    where
        T: TemplateEngine,
        C: Serialize,
    {
        self.router
            .add_context_route(path.as_ref(), engine, template, context)
            .map_err(StaticSiteError::Render)?;
        Ok(self)
    }

    pub fn write_to(&self, output: impl AsRef<Path>) -> Result<(), StaticSiteError> {
        self.router.generate(output).map_err(StaticSiteError::Io)
    }
}
