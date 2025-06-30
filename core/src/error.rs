/*
    appellation: error <module>
    authors: @FL03
*/
//! this module defines the base error types for the `acme` platform
#[cfg(feature = "alloc")]
use alloc::{
    boxed::Box,
    string::{String, ToString},
};

/// a type alias for a [`Result`](core::result::Result) configured with the custom [`Error`]
/// for the crate.
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The [`Error`] type enumerates the various errors that can occur within the `acme` engine.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error("Invalid component name: {0}")]
    InvalidComponentName(String),
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync>),
    #[cfg(feature = "serde")]
    #[error(transparent)]
    DeserializeError(#[from] serde::de::value::Error),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "serde_json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "alloc")]
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[cfg(feature = "alloc")]
impl Error {
    /// create a new [`BoxError`](Error::BoxError) error variant using the provided boxed error.
    pub fn box_error<E>(error: E) -> Self
    where
        E: 'static + core::error::Error + Send + Sync,
    {
        Error::BoxError(Box::new(error))
    }
    /// create a new [`Unknown`](Error::Unknown) error variant using the provided value.
    pub fn unknown<T>(value: T) -> Self
    where
        T: ToString,
    {
        Error::Unknown(value.to_string())
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Unknown(value)
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Unknown(String::from(value))
    }
}
