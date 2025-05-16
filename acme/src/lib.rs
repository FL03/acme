/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme
//! 
//! Welcome to `acme`! This library implements an automated container / context management 
//! system in Rust. The project is technically an extension of the on-going research project, 
//! [`eryon`](https://docs.rs/eryon), essentially materializing these core concepts into a 
//! functional computational framework.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// The core modules of the `acme` library; these modules are re-imported at the top-level of 
/// the library fo convenience.
#[doc(inline)]
pub use acme_core::*;

pub mod prelude {
    pub use acme_core::prelude::*;
}
