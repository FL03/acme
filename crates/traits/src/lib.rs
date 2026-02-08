/*
    Appellation: lib <module>
    Created At: 2026.01.20:14:52:56
    Contrib: @FL03
*/
//! the core modules supporting the `contained` crate focused on establishing foundational
//! primitives and utilities for getter, setters, and wrappers.
#![crate_type = "lib"]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::non_ascii_literal,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}
// modules

pub mod component;
// re-exports
#[doc(inline)]
pub use self::component::Component;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::component::*;
}
