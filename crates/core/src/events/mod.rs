/*
    Appellation: events <module>
    Contrib: @FL03
*/
//! this module implements the various event primitives and utilities necessary for the sdk.
#![cfg(feature = "events")]
#[doc(inline)]
pub use self::{event_base::EventBase, traits::*, types::*};

mod event_base;

mod impls {
    mod impl_event_base;
}

mod traits {
    #[doc(inline)]
    pub use self::{event_type::*, raw_event::*};

    mod event_type;
    mod raw_event;
}

mod types {
    #[doc(inline)]
    pub use self::kinds::*;

    mod kinds;
}
// prelude {local}
pub(crate) mod prelude {
    pub use super::event_base::*;
    pub use super::traits::*;
    pub use super::types::*;
}
