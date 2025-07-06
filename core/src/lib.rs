/*
    Appellation: acme-core <library>
    Contrib: @FL03
*/
//! The _core_ modules of the `acme` library; these modules are re-exported at the top-level of
//! the library for convenience.
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
pub use self::{
    comp::prelude::*, consts::*, error::*, events::prelude::*, traits::*, types::*, utils::*,
};

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub(crate) mod seal;
}

pub mod comp;
pub mod error;
pub mod events;

pub mod consts {
    #[doc(hidden)]
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
}

pub mod traits {
    //! this module provides common traits and interfaces for the `acme` engine
    #[doc(inline)]
    pub use self::prelude::*;

    mod context;
    mod handle;

    mod prelude {
        #[doc(inline)]
        pub use super::context::*;
        #[doc(inline)]
        pub use super::handle::*;
    }
}

pub mod types {
    //! this module implements various types used for the `acme` engine
    #[doc(inline)]
    pub use self::prelude::*;

    mod container;

    mod prelude {
        #[doc(inline)]
        pub use super::container::*;
    }
}

pub mod utils {
    //! this module defines additional utilities for the `acme` engine
    #[doc(inline)]
    pub use self::prelude::*;

    mod generate;

    mod prelude {
        #[doc(inline)]
        pub use super::generate::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::comp::prelude::*;
    #[doc(no_inline)]
    pub use crate::consts::*;
    #[doc(no_inline)]
    pub use crate::events::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::*;
    #[doc(no_inline)]
    pub use crate::types::*;
    #[doc(no_inline)]
    pub use crate::utils::*;
}
