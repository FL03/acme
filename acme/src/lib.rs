/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme
//!
//! Welcome to `acme`! This crate is designed to be an efficient, flexible, and secure engine
//! for automatically aggreating information from various sources, processing it, and
//! distributing it to the appropriate consumers.
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use acme_core::*;

pub mod prelude {
    pub use acme_core::prelude::*;
}
