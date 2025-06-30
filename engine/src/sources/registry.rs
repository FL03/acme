/*
    appellation: registry <module>
    authors: @FL03
*/

/// The [`SourceRegistry`] is a store for the various data sources that is directly managed
/// by the [`SourceManager`](super::SourceManager). It is used to register and manage the
/// sources available in the system.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct SourceRegistry;

impl SourceRegistry {
    /// create a new instance of the [`SourceRegistry`].
    pub const fn new() -> Self {
        SourceRegistry
    }
}
