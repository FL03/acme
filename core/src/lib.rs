/*
    Appellation: acme-core <library>
    Contrib: @FL03
*/
//! The core modules for the `acme` framework.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::error::*;

pub mod error;

pub mod consts {}

pub mod traits {}

pub mod types {}

pub mod utils {}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;
}
