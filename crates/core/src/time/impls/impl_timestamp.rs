/*
    Appellation: impl_timestamp <module>
    Created At: 2026.02.07:18:18:47
    Contrib: @FL03
*/
use crate::time::ts::Timestamp;
use crate::time::{Now, RawTimestamp};

impl<T: RawTimestamp> Timestamp<T> {
    /// create a new instance of [`Timestamp`] with the given value.
    pub const fn new(ts: T) -> Self {
        Self(ts)
    }
    /// a convenience method to get the current timestamp; requires that the inner type
    /// implement the [`Now`] trait.
    pub fn now() -> Self
    where
        Timestamp<T>: Now<Output = Self>,
    {
        <Self as Now>::now()
    }
    /// Get an immutable reference to the current timestamp.
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// Get a mutable reference to the current timestamp.
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    #[inline]
    /// consumes the current instance and returns the inner value.
    pub fn value(self) -> T {
        self.0
    }
    /// [`replace`](core::mem::replace) the current value with a new one and return the old one
    pub const fn replace(&mut self, value: T) -> T {
        core::mem::replace(self.get_mut(), value)
    }
    #[inline]
    /// update the current value and return a mutable reference to the current instance.
    pub fn set(&mut self, ts: T) {
        self.0 = ts
    }
    /// [`swap`](core::mem::swap) the current value with another and return a mutable reference to self
    pub const fn swap(&mut self, ts: &mut T) {
        core::mem::swap(self.get_mut(), ts)
    }
    #[inline]
    /// [`take`](core::mem::take) the current value and return it, leaving the logical
    /// [`default`](Default::default) in its place.
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// consumes the current instance to create another with the given value
    #[inline]
    pub fn with<U: RawTimestamp>(self, ts: U) -> Timestamp<U> {
        Timestamp(ts)
    }
    /// applies a function onto the current value and returns a new instance with the result
    pub fn map<U, F>(self, f: F) -> Timestamp<U>
    where
        F: FnOnce(T) -> U,
        U: RawTimestamp,
    {
        Timestamp(f(self.value()))
    }
    /// returns a new instance of the [`Timestamp`] with the current value updated using the given function
    pub fn map_inplace<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T),
    {
        f(self.get_mut());
        self
    }
    /// updates the timestamp to reflect _now_ and return the previous timestamp
    pub fn update(&mut self) -> T
    where
        Timestamp<T>: Now<Output = Self>,
    {
        let now = <Self as Now>::now();
        // replace the current value with the new one
        self.replace(now.value())
    }
    /// returns a new instance of the [`Timestamp`] containing an immutable reference to the
    /// inner value.
    pub const fn view(&self) -> Timestamp<&T> {
        Timestamp(self.get())
    }
    /// returns a new instance of the [`Timestamp`] containing a mutable reference to the inner
    /// value.
    pub const fn view_mut(&mut self) -> Timestamp<&mut T> {
        Timestamp(self.get_mut())
    }
}

impl<T> Default for Timestamp<T>
where
    T: RawTimestamp,
    Timestamp<T>: Now<Output = Self>,
{
    fn default() -> Self {
        Self::now()
    }
}

impl<T: RawTimestamp> AsRef<T> for Timestamp<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: RawTimestamp> AsMut<T> for Timestamp<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: RawTimestamp> core::borrow::Borrow<T> for Timestamp<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T: RawTimestamp> core::borrow::BorrowMut<T> for Timestamp<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: RawTimestamp> core::ops::Deref for Timestamp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: RawTimestamp> core::ops::DerefMut for Timestamp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: RawTimestamp> From<T> for Timestamp<T> {
    fn from(value: T) -> Self {
        Timestamp(value)
    }
}

contained::fmt_wrapper! {
    impl Timestamp<T> {
        Display,
        Binary,
        Octal,
        LowerHex,
        UpperHex,
        LowerExp,
        UpperExp,
        Pointer,
    }
}
