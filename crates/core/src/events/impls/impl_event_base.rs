/*
    Appellation: impl_event_base <module>
    Created At: 2026.02.07:23:58:40
    Contrib: @FL03
*/
use crate::events::event_base::EventBase;

use crate::Timestamp;
use crate::events::EventType;
use alloc::vec::Vec;

impl<K, T> EventBase<T, K>
where
    K: EventType,
{
    /// initializes a new [`EventBase`] with the provided data, an empty message, and the current timestamp.
    pub fn new<M>(data: T, message: M) -> Self
    where
        M: IntoIterator<Item = u8>,
    {
        Self {
            data,
            message: Vec::from_iter(message),
            timestamp: Timestamp::now(),
            _type: core::marker::PhantomData::<K>,
        }
    }
    /// initializes a new [`EventBase`] with the provided data, an empty message, and the current timestamp.
    pub fn from_data(data: T) -> Self {
        Self {
            data,
            message: Vec::new(),
            timestamp: Timestamp::now(),
            _type: core::marker::PhantomData::<K>,
        }
    }
    /// returns a reference to the data contained within the event.
    pub const fn data(&self) -> &T {
        &self.data
    }
}
