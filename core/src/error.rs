#[allow(dead_code)]
/// a type alias for a [`Result`] type pre-configured with the [`CoreError`] type
pub(crate) type CoreResult<T = ()> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid component name: {0}")]
    InvalidComponentName(&'static str),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] alloc::boxed::Box<dyn core::error::Error>),
    #[error("Unknown Error: {0}")]
    Unknown(&'static str),
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::Unknown(value)
    }
}
