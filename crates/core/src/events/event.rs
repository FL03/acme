/*
    Appellation: event <module>
    Contrib: @FL03
*/
use core::marker::PhantomData;
use scsys::Timestamp;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventBase<T, K> {
    pub(crate) data: T,
    pub(crate) message: Vec<u8>,
    pub(crate) timestamp: Timestamp,
    pub(crate) _type: PhantomData<K>,
}

impl<K, T> EventBase<T, K> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            message: Vec::new(),
            timestamp: Timestamp::now(),
            _type: PhantomData::<K>,
        }
    }
}
