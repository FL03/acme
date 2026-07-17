/*
    Appellation: acme-engine <library>
    Contrib: @FL03
*/
//! This crate provides the
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(all(feature = "std", feature = "alloc")))]
compile_error! {
    "The `acme-engine` crate requires either the `std` or `alloc` feature to be enabled."
}

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    engine::*,
    error::*,
    pipes::prelude::*,
    scheduler::Scheduler,
    sources::{SourceManager, SourceRegistry},
    traits::*,
    types::*,
};

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub(crate) mod seal;
}

pub mod engine;
pub mod error;
pub mod pipes;
pub mod scheduler;
pub mod sources;

mod traits {
    //! this module provides common traits and interfaces for the `acme` engine
    #[doc(inline)]
    pub use self::prelude::*;

    mod engine;

    mod prelude {
        #[doc(inline)]
        pub use super::engine::*;
    }
}

mod types {
    //! this module implements various types used for the `acme` engine
    #[doc(inline)]
    pub use self::prelude::*;

    mod aliases;

    mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::engine::*;
    #[doc(no_inline)]
    pub use crate::pipes::prelude::*;
    #[doc(no_inline)]
    pub use crate::scheduler::*;
    #[doc(no_inline)]
    pub use crate::sources::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::*;
    #[doc(no_inline)]
    pub use crate::types::*;
}
