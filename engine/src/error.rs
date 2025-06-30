/*
    appellation: error <module>
    authors: @FL03
*/
//! this module defines the [`EngineError`] type to enumerate the various errors that can occur
//! within the engine.

#[allow(dead_code)]
/// a type alias for a [`Result`](core::result::Result) that uses the[`EngineError`]
pub(crate) type Result<T = ()> = core::result::Result<T, EngineError>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum EngineError {
    #[cfg(feature = "alloc")]
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error(transparent)]
    CoreError(#[from] acme_core::error::Error),
}

#[cfg(feature = "alloc")]
impl From<EngineError> for acme_core::error::Error {
    fn from(err: EngineError) -> Self {
        match err {
            EngineError::CoreError(e) => e,
            _ => acme_core::error::Error::box_error(err),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<String> for EngineError {
    fn from(value: String) -> Self {
        acme_core::Error::unknown(value).into()
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for EngineError {
    fn from(value: &str) -> Self {
        acme_core::Error::unknown(value).into()
    }
}
