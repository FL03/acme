/*
    appellation: error <module>
    authors: @FL03
*/
//! this module defines the [`Error`] enum and related types for error handling within the
//! crate.
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// a type alias for a [`Result`](core::result::Result) configured to use the custom [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// The [`Error`] implementation defines the possible errors that can occur within the crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    // external errors
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    // core errors
    #[error(transparent)]
    AddrParseError(#[from] core::net::AddrParseError),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),
    // std-dependent errors
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    // alloc-dependent variants
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[cfg(feature = "alloc")]
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

impl Error {
    #[cfg(feature = "alloc")]
    /// a functional constructor for the [`BoxError`](Self::BoxError) variant
    pub fn boxed<E>(error: E) -> Self
    where
        E: core::error::Error + Send + Sync + 'static,
    {
        Self::BoxError(Box::new(error))
    }
    #[cfg(feature = "alloc")]
    /// a functional constructor for the [`Unknown`](Self::Unknown) variant
    pub fn unknown<E>(message: E) -> Self
    where
        E: alloc::string::ToString,
    {
        Self::Unknown(message.to_string())
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::unknown(value)
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Unknown(value)
    }
}
