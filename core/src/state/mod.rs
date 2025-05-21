/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::state::*;

pub mod state;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::state::*;
    #[doc(inline)]
    pub use super::{RawState, Stateful};
}

/// a trait for denoting stateful entities
pub trait Stateful<Q> where Q: RawState {
    type State<R>;

    fn state(&self) -> Self::State<&Q>;

    fn state_mut(&mut self) -> Self::State<&mut Q>;
}

/// [RawState]
pub trait RawState {
    type Inner;
}

/*
 ************* Implementations *************
*/
impl<Q> RawState for State<Q> {
    type Inner = Q;
}

impl<Q, T> Stateful<Q> for State<Q>
where
    Q: RawState<Inner = T>,
{
    type State<R> = State<R>;

    fn state(&self) -> Self::State<&Q> {
        self.view()
    }

    fn state_mut(&mut self) -> Self::State<&mut Q> {
        self.view_mut()
    }
}
