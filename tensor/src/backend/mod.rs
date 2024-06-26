/*
    Appellation: backend <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Backend
//!
//!
pub use self::devices::*;

pub(crate) mod devices;

pub mod cpu;

pub trait Backend {}

pub trait BackendStorage {
    type Backend: Backend;
}

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::devices::Device;
    pub use super::{Backend, BackendStorage};
}

#[cfg(test)]
mod tests {}
