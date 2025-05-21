/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

/// The [`State`] type is a generic wrapper for any type `Q` that implements the [`RawState`] 
/// trait.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
#[serde(default, transparent)]
pub struct State<Q>(pub Q);

impl<Q> State<Q> {
    /// returns a new instance of the state initialized with the default value
    pub fn new() -> Self
    where
        Q: Default,
    {
        Self(Q::default())
    }
    /// returns a new instance of the state with the given value
    pub fn from_state(value: Q) -> Self {
        Self(value)
    }
    /// check if the current instance is of the given type
    pub fn is_type_of<R: 'static>(&self) -> bool
    where
        Q: 'static,
    {
        use core::any::TypeId;
        TypeId::of::<PhantomData<Q>>() == TypeId::of::<PhantomData<R>>()
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &Q {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut Q {
        &mut self.0
    }
    /// consumes the current instance to return the inner value
    pub fn into_value(self) -> Q {
        self.0
    }
    /// applies the given function to the inner value and returns a new instance of the state
    /// that wraps the output
    pub fn map<R, F>(self, f: F) -> State<R>
    where
        F: FnOnce(Q) -> R,
    {
        State(f(self.0))
    }
    /// uses the [`replace`](core::mem::replace) method to update and return the inner value
    pub fn replace(&mut self, value: Q) -> Q {
        core::mem::replace(self.get_mut(), value)
    }
    /// update the innerstate before returing a mutable reference to the wrapper
    pub fn set(&mut self, value: Q) -> &mut Self {
        *self.get_mut() = value;
        self
    }
    /// uses the [`take`](core::mem::take) method to replace the inner value with the default 
    /// value to return its previous value
    pub fn take(&mut self) -> Q
    where
        Q: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// consumes the current instance to create another with the given value
    pub fn with(self, value: Q) -> Self {
        Self(value)
    }
    /// returns a new instance of the state with a referenced inner valued
    pub fn view(&self) -> State<&Q> {
        State(self.get())
    }

    pub fn view_mut(&mut self) -> State<&mut Q> {
        State(self.get_mut())
    }
}

impl<Q> AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        self.get()
    }
}

impl<Q> AsMut<Q> for State<Q> {
    fn as_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::borrow::Borrow<Q> for State<Q> {
    fn borrow(&self) -> &Q {
        self.get()
    }
}

impl<Q> core::borrow::BorrowMut<Q> for State<Q> {
    fn borrow_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::ops::Deref for State<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<Q> core::ops::DerefMut for State<Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<Q> From<Q> for State<Q> {
    fn from(value: Q) -> Self {
        Self(value)
    }
}

fmt_wrapper! {
    State<Q> (
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