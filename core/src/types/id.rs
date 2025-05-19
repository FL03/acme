/*
    Appellation: atomic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// A generic identifier
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
#[serde(default, transparent)]
pub struct Id<T = usize>(pub T);

impl<T> Id<T> {
    /// Create a new identifier with the default value
    pub fn new() -> Self
    where
        T: Default,
    {
        Self(T::default())
    }
    /// create a new identifier from the given value
    pub fn from_value(id: T) -> Self {
        Self(id)
    }
    #[cfg(feature = "rand")]
    pub fn random() -> Self
    where
        rand_distr::StandardUniform: rand_distr::Distribution<T>,
    {
        use rand::Rng;
        let mut rng = rand::rng();
        Self::from_value(rng.random())
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// consumes the current instance to return the inner value
    pub fn into_inner(self) -> T {
        self.0
    }
    /// use the [`replace`](core::mem::replace) method to update and return the inner value
    pub fn replace(&mut self, id: T) -> T {
        core::mem::replace(self.get_mut(), id)
    }
    /// mutate the inner value and return a mutable reference to the wrapper for chaining
    pub fn set(&mut self, id: T) -> &mut Self {
        *self.get_mut() = id;
        self
    }
    /// takes the inner value and replaces it with the default value
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// consumes the current instance to replace it with another.
    pub fn with<U>(self, id: U) -> Id<U> {
        Id(id)
    }
}

impl Id<usize> {
    pub fn atomic() -> Self {
        use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self(COUNTER.fetch_add(1, Relaxed))
    }

    pub fn next(&self) -> Self {
        Self::atomic()
    }
}

#[cfg(feature = "uuid")]
impl Id<uuid::Uuid> {
    #[cfg(all(feature = "rng", feature = "uuid"))]
    pub fn v4() -> Self {
        Self::from_value(uuid::Uuid::new_v4())
    }
}

impl<T: Default> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AsRef<T> for Id<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Id<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::borrow::Borrow<T> for Id<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for Id<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Id<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Id<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


fmt_wrapper! {
    Id<T> (
        Binary("{:b}"),
        Debug("{:?}"),
        Display("{}"),
        LowerExp("{:e}"),
        LowerHex("{:x}"),
        Octal("{:o}"),
        UpperExp("{:E}"),
        UpperHex("{:X}"),
    )
}

impl<S> PartialEq<S> for Id
where
    usize: PartialEq<S>,
{
    fn eq(&self, other: &S) -> bool {
        self.0.eq(other)
    }
}
