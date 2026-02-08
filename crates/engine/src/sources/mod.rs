/*
    appellation: sources <module>
    authors: @FL03
*/
//! this module defines the varioussources the system is compatible with as well as a
//! [`SourceManager`] for handling them
#[doc(inline)]
pub use self::{manager::SourceManager, registry::SourceRegistry};

pub mod manager;
pub mod registry;

pub(crate) mod prelude {
    pub use super::manager::*;
    pub use super::registry::*;
}
