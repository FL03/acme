/*
    Appellation: event <module>
    Contrib: @FL03
*/

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventBase<K, T> {
    pub(crate) data: T,
    pub(crate) message: Vec<u8>,
    pub(crate) timestamp: u128,
    pub(crate) _type: core::marker::PhantomData<K>,
}

impl<K, T> EventBase<K, T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            message: Vec::new(),
            timestamp: crate::utils::systime(),
            _type: core::marker::PhantomData,
        }
    }
}
