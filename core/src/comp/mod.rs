/*
    Appellation: comp <module>
    Contrib: @FL03
*/
//! This module implements the [`Component`] and related types
#[doc(inline)]
pub use self::component::*;

pub mod component;
pub mod interface;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::component::*;
}
