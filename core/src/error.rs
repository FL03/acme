/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::kinds::*;

pub(crate) mod kinds;

pub type Result<T = ()> = core::result::Result<T, Error>;

use core::fmt::{self, Debug, Display};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
pub struct Error<K = String> {
    kind: ErrorKind<K>,
    message: String,
}

impl<K> Error<K> {
    pub fn new(kind: ErrorKind<K>, msg: impl ToString) -> Self {
        Self {
            kind,
            message: msg.to_string(),
        }
    }
    /// Get an owned reference to the error kind
    pub fn kind(&self) -> &ErrorKind<K> {
        &self.kind
    }
    /// Get an owned reference to the error message
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Set the error message
    pub fn set_message(&mut self, msg: impl ToString) {
        self.message = msg.to_string();
    }
    /// Consume the error and return the message
    pub fn into_message(self) -> String {
        self.message
    }
    /// A functional method for setting the error kind
    pub fn with_kind(mut self, kind: ErrorKind<K>) -> Self {
        self.kind = kind;
        self
    }
    /// A functional method for setting the error message
    pub fn with_message(mut self, msg: impl ToString) -> Self {
        self.message = msg.to_string();
        self
    }
}

impl<K> Display for Error<K>
where
    K: ToString,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl<K> std::error::Error for Error<K> where K: Debug + Display {}

impl<K> From<ErrorKind<K>> for Error<K> {
    fn from(kind: ErrorKind<K>) -> Self {
        Self::new(kind, "")
    }
}

impl<K, T> From<std::sync::TryLockError<T>> for Error<K> {
    fn from(err: std::sync::TryLockError<T>) -> Self {
        Self::new(ErrorKind::Sync(SyncError::TryLock), err.to_string())
    }
}

macro_rules! err_from {
    ($kind:expr, $t:ty) => {
        impl<E> From<$t> for Error<E> {
            fn from(err: $t) -> Self {
                Self::new($kind, err.to_string())
            }
        }
    };
    ($kind:expr => ($($t:ty),*)) => {
        $(err_from!($kind, $t);)*
    };
}

err_from!(ErrorKind::External(ExternalError::Unknown) => (&str, String, Box<dyn std::error::Error>));
