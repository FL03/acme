/*
    appellation: interface <module>
    authors: @FL03
*/

pub type SharedInterface<T> = std::sync::Arc<Interface<T>>;

/// An [`Interface`] is a generic wrapper dedicated to providing a common abstract for
/// applications and other components to interact with each other.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, transparent)
)]
#[repr(transparent)]
pub struct Interface<T> {
    pub(crate) inner: T,
}

contained::fmt_wrapper! {
    Interface<T>.inner::(
        Debug,
        Display,
        Binary,
        LowerExp,
        UpperExp,
        LowerHex,
        UpperHex,
        Octal,
        Pointer,    
    )
}
