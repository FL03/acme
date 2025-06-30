/*
    appellation: pipes <module>
    authors: @FL03
*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct PipeRouter;

impl PipeRouter {
    /// create a new instance of the [`PipeRouter`].
    pub const fn new() -> Self {
        Self
    }
}
