/*
    appellation: scheduler <module>
    authors: @FL03
*/

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
    pub async fn start(&self, _source_manager: crate::sources::SourceManager) {
        // Implementation for starting the scheduler goes here.
        // This is a placeholder for now.
        todo!("Scheduler start logic not implemented yet");
    }
}
