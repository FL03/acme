/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! acme is an automated component management engine.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use acme_core::*;

pub mod prelude {
    pub use acme_core::prelude::*;
}
