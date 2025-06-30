/*
    appellation: scheduler <module>
    authors: @FL03
*/
use crate::sources::SourceManager;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct Scheduler;

impl Scheduler {
    /// create a new instance of the [`Scheduler`].
    pub const fn new() -> Self {
        Self
    }

    /// start the scheduler with the provided [`SourceManager`].
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, level = "trace"))]
    pub async fn start(&self, _ctx: SourceManager) {
        // Implementation for starting the scheduler goes here.
        // This is a placeholder for now.
        todo!("Scheduler start logic not implemented yet");
    }
}
