/*
    Appellation: events <module>
    Contrib: @FL03
*/
//! this module implements the various event primitives and utilities necessary for the sdk.
#[doc(inline)]
pub use self::event::EventBase;

pub(crate) mod event;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::event::*;
    #[doc(inline)]
    pub use super::{RawEvent, RawEventKind};
}


pub trait RawEventKind {

    private!();
}

pub trait EventKind: RawEventKind {
    fn kind() -> &'static str;
}

pub trait RawEvent {
    type Kind: RawEventKind;
}