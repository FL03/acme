/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{dtype::*, id::*, kinds::*, order::*, tensors::*};

pub(crate) mod dtype;
pub(crate) mod id;
pub(crate) mod kinds;
pub(crate) mod order;
pub(crate) mod tensors;

pub(crate) mod prelude {
    pub use super::dtype::*;
    pub use super::id::*;
    pub use super::kinds::*;
    pub use super::order::*;
    pub use super::tensors::*;
}