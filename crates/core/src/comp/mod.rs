/*
    Appellation: comp <module>
    Contrib: @FL03
*/
//! This module implements the [`Component`] and related types
#[doc(inline)]
pub use self::{component::*, interface::*};

mod component;
mod interface;

// prelude (local)
pub(crate) mod prelude {
    pub use super::component::*;
}
