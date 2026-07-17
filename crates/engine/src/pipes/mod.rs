/*
    appellation: sources <module>
    authors: @FL03
*/
//! this module defines automation pipelines for the aggregation and processing of data
#[doc(inline)]
pub use self::router::PipeRouter;

mod router;

pub(crate) mod prelude {
    pub use super::router::*;
}
