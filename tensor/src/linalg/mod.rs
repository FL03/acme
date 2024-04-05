/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Linear Algebra
//!
//!
pub mod tri;
pub mod uplo;

pub trait Inverse {
    fn inv(&self) -> Self;
}

/// Matrix multiplication
pub trait Matmul<Rhs = Self> {
    type Output;

    fn matmul(&self, rhs: &Rhs) -> Self::Output;
}

pub(crate) mod prelude {
    pub use super::uplo::UPLO;
    pub use super::{Inverse, Matmul};
}

#[cfg(test)]
mod tests {}
