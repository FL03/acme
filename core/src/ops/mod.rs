/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::kinds::*;

pub(crate) mod kinds;

pub mod binary;
pub mod unary;

pub trait ApplyTo<T> {
    type Output;

    fn apply_to(&self, other: T) -> Self::Output;
}

pub trait ApplyWith<T> {
    type Output;
    type With;

    fn apply_with(&self, other: T, with: Self::With) -> Self::Output;
}

pub trait IntoOp {
    fn into_op(self) -> Op;
}

impl<S> IntoOp for S
where
    S: Into<Op>,
{
    fn into_op(self) -> Op {
        self.into()
    }
}

pub trait Operation {
    type Kind;

    fn kind(&self) -> Self::Kind;
}

pub(crate) mod prelude {
    pub use super::binary::*;
    pub use super::kinds::Op;
    pub use super::unary::*;
    pub use super::Operation;
}
