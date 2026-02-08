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
#![cfg_attr(all(feature = "nightly", feature = "alloc"), feature(allocator_api))]
// compile-time checks
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "Either the `std` or `alloc` feature must be enabled for the crate to compile" }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub(crate) mod seal;
}
// modules
pub mod comp;
pub mod consts;
pub mod error;
pub mod events;
pub mod time;

mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    mod container;

    mod prelude {
        #[doc(inline)]
        pub use super::container::*;
    }
}

mod utils {
    #[doc(inline)]
    pub use self::prelude::*;

    mod generate;

    mod prelude {
        #[doc(inline)]
        pub use super::generate::*;
    }
}
// re-exports
#[doc(inline)]
pub use acme_traits::prelude::*;
#[doc(inline)]
pub use self::{
    comp::prelude::*, consts::*, error::*, events::prelude::*, time::Timestamp,
    types::*, utils::*,
};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::comp::prelude::*;
    pub use crate::consts::*;
    pub use crate::events::prelude::*;
    pub use crate::time::prelude::*;
    pub use crate::types::*;
    pub use crate::utils::*;
}
