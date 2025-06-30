/*
    appellation: engine <module>
    authors: @FL03
*/
use crate::pipes::PipeRouter;
use crate::scheduler::Scheduler;
use crate::sources::SourceManager;
use std::sync::Arc;
#[cfg(not(feature = "tokio"))]
use std::sync::RwLock;
#[cfg(feature = "tokio")]
use tokio::sync::RwLock;

/// The [`Engine`] implementation focuses on aggregating information from various sources,
/// synchronizing their content, and providing a unified, extensible interface for data.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct Engine {
    pub(crate) controller: SourceManager,
    pub(crate) scheduler: Scheduler,
    pub(crate) pipe_router: Arc<RwLock<PipeRouter>>,
}

impl Engine {
    pub fn new() -> crate::Result<Self> {
        let pipe_router = PipeRouter::new();

        let controller = SourceManager::from_router(pipe_router.clone());
        let scheduler = Scheduler::new();
        let pipe_router = Arc::new(RwLock::new(pipe_router));
        Ok(Self {
            controller,
            scheduler,
            pipe_router,
        })
    }
    /// returns an immutable reference to the current [`SourceManager`].
    pub const fn controller(&self) -> &SourceManager {
        &self.controller
    }
    /// returns a mutable reference to the current [`SourceManager`].
    pub const fn controller_mut(&mut self) -> &mut SourceManager {
        &mut self.controller
    }
    /// returns a reference to the [`PipeRouter`]
    pub const fn pipe_router(&self) -> &Arc<RwLock<PipeRouter>> {
        &self.pipe_router
    }
    /// returns a mutable reference to the [`PipeRouter`]
    pub fn pipe_router_mut(&mut self) -> Arc<RwLock<PipeRouter>> {
        Arc::clone(&self.pipe_router)
    }
    /// returns a reference to the [`Scheduler`]
    pub const fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }
    /// returns a mutable reference to the [`Scheduler`]
    pub const fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }
    /// starts the engine
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, level = "trace"))]
    pub async fn run(&self) -> crate::Result<()> {
        self.scheduler().start(self.controller().clone()).await;
        Ok(())
    }
}
