/*
    Appellation: acme-core <library>
    Contrib: @FL03
*/
//! The core modules for the `acme` framework.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    comp::prelude::*, consts::*, error::*, events::prelude::*, traits::prelude::*,
    types::prelude::*, utils::prelude::*,
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
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod context;
    pub mod handle;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::context::*;
        #[doc(inline)]
        pub use super::handle::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod container;
    pub mod id;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::container::*;
        #[doc(inline)]
        pub use super::id::*;
    }
}

pub mod utils {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod generate;
    pub mod time;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::generate::*;
        #[doc(inline)]
        pub use super::time::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::comp::prelude::*;
    #[doc(no_inline)]
    pub use crate::consts::*;
    #[doc(no_inline)]
    pub use crate::error::*;
    #[doc(no_inline)]
    pub use crate::events::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
    #[doc(no_inline)]
    pub use crate::utils::prelude::*;
}
