/*
    Appellation: wrapper <module>
    Contrib: @FL03
*/

#[allow(unused_macros)]
macro_rules! wrapper {
    ($($S:ident($vis:vis $T:ident)),* $(,)?) => {
        $(
            wrapper!(@impl $S($vis $T));
        )*
    };
    (@impl $S:ident($vis:vis $T:ident)) => {
        #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[repr(transparent)]
        #[serde(default, transparent)]
        pub struct $S<$T>(pub $T);

        impl<$T> $S<$T> {
            /// returns a new instance initialized with the default value
            pub fn new() -> Self
            where
                $T: Default,
            {
                Self($T::default())
            }
            /// returns an immutable reference to the inner value
            pub const fn get(&self) -> &$T {
                &self.0
            }
            /// returns a mutable reference to the inner value
            pub const fn get_mut(&mut self) -> &mut $T {
                &mut self.0
            }
            /// consumes the current instance to return the inner value
            pub fn into_inner(self) -> $T {
                self.0
            }
            /// uses the [`replace`](core::mem::replace) method to update and return the inner value
            pub fn replace(&mut self, value: $T) -> $T {
                core::mem::replace(self.get_mut(), value)
            }
            /// update the innerstate before returing a mutable reference to the wrapper
            pub fn set(&mut self, value: $T) -> &mut Self {
                *self.get_mut() = value;
                self
            }
            /// uses the [`take`](core::mem::take) method to replace the inner value with the default 
            /// value to return its previous value
            pub fn take(&mut self) -> $T
            where
                $T: Default,
            {
                core::mem::take(self.get_mut())
            }
            /// consumes the current instance to create another with the given value
            pub fn with(self, value: $T) -> Self {
                Self(value)
            }
            /// applies the given function to the inner value and returns a new instance with
            /// the result
            pub fn map<R, F>(self, f: F) -> $S<R>
            where
                F: FnOnce($T) -> R,
            {
                $S(f(self.0))
            }
        }

        impl<$T> AsRef<$T> for $S<$T> {
            fn as_ref(&self) -> &$T {
                self.get()
            }
        }
        
        impl<$T> AsMut<$T> for $S<$T> {
            fn as_mut(&mut self) -> &mut $T {
                self.get_mut()
            }
        }
        
        impl<$T> core::borrow::Borrow<$T> for $S<$T> {
            fn borrow(&self) -> &$T {
                self.get()
            }
        }
        
        impl<$T> core::borrow::BorrowMut<$T> for $S<$T> {
            fn borrow_mut(&mut self) -> &mut $T {
                self.get_mut()
            }
        }
        
        impl<$T> core::ops::Deref for $S<$T> {
            type Target = $T;
        
            fn deref(&self) -> &Self::Target {
                self.get()
            }
        }
        
        impl<$T> core::ops::DerefMut for $S<$T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.get_mut()
            }
        }
        
        impl<$T> From<$T> for $S<$T> {
            fn from(value: $T) -> Self {
                Self(value)
            }
        }
    };
}