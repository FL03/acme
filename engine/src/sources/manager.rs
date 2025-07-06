/*
    appellation: manager <module>
    authors: @FL03
*/
use super::SourceRegistry;
use crate::pipes::PipeRouter;

/// The [`SourceManager`] is responsible for managing the current data sources
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct SourceManager {
    pub(crate) registry: SourceRegistry,
    pub(crate) router: PipeRouter,
}

impl SourceManager {
    /// create a new instance of the [`SourceManager`].
    pub const fn new() -> Self {
        Self {
            registry: SourceRegistry::new(),
            router: PipeRouter::new(),
        }
    }
    /// create a new [`SourceManager`] with the provided [`SourceRegistry`] and the default
    /// [`PipeRouter`].
    pub fn from_registry(registry: SourceRegistry) -> Self {
        Self {
            registry,
            router: PipeRouter::default(),
        }
    }
    /// create a new [`SourceManager`] with the provided [`PipeRouter`] and the default
    /// [`SourceRegistry`].
    pub fn from_router(router: PipeRouter) -> Self {
        Self {
            registry: SourceRegistry::default(),
            router,
        }
    }
    /// returns an immutable reference to the current [`SourceRegistry`].
    pub const fn registry(&self) -> &SourceRegistry {
        &self.registry
    }
    /// returns a mutable reference to the current [`SourceRegistry`].
    pub const fn registry_mut(&mut self) -> &mut SourceRegistry {
        &mut self.registry
    }
    /// returns a reference to the [`PipeRouter`] used by this [`SourceManager`].
    pub const fn router(&self) -> &PipeRouter {
        &self.router
    }
    /// returns a mutable reference to the [`PipeRouter`] used by this [`SourceManager`].
    pub fn router_mut(&mut self) -> &mut PipeRouter {
        &mut self.router
    }
    /// overwrite the current [`SourceRegistry`] with the provided one and return a mutable
    /// reference to self
    pub fn set_registry(&mut self, registry: SourceRegistry) -> &mut Self {
        self.registry = registry;
        self
    }
    /// consumes the current instance to create another with the given registry.
    pub const fn with_registry(self, registry: SourceRegistry) -> Self {
        Self { registry, ..self }
    }
    /// consumes the current instance to create another with the given [`PipeRouter`].
    pub const fn with_router(self, router: PipeRouter) -> Self {
        Self { router, ..self }
    }
}
