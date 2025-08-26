/*
    appellation: format <module>
    authors: @FL03
*/

/// A macro for implementing formatting traits onto so-called wrapper types; i.e. any struct
/// capable of implementing the `#[repr(transparent)]` attribute.
///
/// # Examples
///
/// ```ignore
/// fmt_wrapper! {
///     MyWrapper<T>::inner {
///         Debug,
///         Display,
///         Binary,
///         LowerExp,
///         UpperExp,
///         LowerHex,
///         UpperHex,
///     }
/// }
/// ```
///
/// or, for tuple structs:
///
/// ```ignore
/// fmt_wrapper! {
///     MyWrapper<T>(Debug, Display, Binary, LowerExp, UpperExp, LowerHex, UpperHex)
/// }
///
macro_rules! fmt_wrapper {
    ($target:ident<$T:ident>::$field:ident { $($trait:ident),* $(,)?}) => {
        $(
            impl<$T> ::core::fmt::$trait for $target<$T>
            where
                $T: ::core::fmt::$trait,
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::$trait::fmt(&self.$field, f)
                }
            }
        )*
    };
    ($target:ident<$T:ident>($($trait:ident),* $(,)?)) => {
        $(
            impl<$T> ::core::fmt::$trait for $target<$T>
            where
                $T: ::core::fmt::$trait,
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::$trait::fmt(&self.0, f)
                }
            }
        )*
    };
}

#[allow(unused_macros)]
macro_rules! wrapper {
    ($target:ident<$T:ident> { $field:ident }) => {
        impl<$T> $target<$T> {
            pub const fn new($field: $T) -> Self {
                Self { $field }
            }
            /// consumes the wrapper to return the inner value
            #[inline]
            pub fn into_inner(self) -> $T {
                self.$field
            }
            /// returns a reference to the inner value
            pub const fn get(&self) -> &$T {
                &self.$field
            }
            /// returns a mutable reference to the inner value
            pub const fn get_mut(&mut self) -> &mut $T {
                &mut self.$field
            }
        }

        wrapper!(@impl $target<$T>);
    };
    ($target:ident<$T:ident>()) => {
        impl<$T> $target<$T> {
            pub const fn new(value: $T) -> Self {
                Self(value)
            }
            /// consumes the wrapper to return the inner value
            #[inline]
            pub fn into_inner(self) -> $T {
                self.0
            }
            /// returns a reference to the inner value
            pub const fn get(&self) -> &$T {
                &self.0
            }
            /// returns a mutable reference to the inner value
            pub const fn get_mut(&mut self) -> &mut $T {
                &mut self.0
            }
        }

        wrapper!(@impl $target<$T>);
    };
    (@impl $target:ident<$T:ident>) => {
        impl<$T> $target<$T> {
            /// apply the closure `f` onto the inner value and capture the result
            pub fn map<F, U>(&self, f: F) -> $target<U>
            where
                F: FnOnce(&$T) -> U,
            {
                $target::new(f(self.get()))
            }
            /// [`replace`](core::mem::replace) the inner value with a new one, returning the old value
            pub const fn replace(&mut self, value: $T) -> $T {
                ::core::mem::replace(sefl.get_mut(), value)
            }
            /// update the inner value
            #[inline]
            pub fn set(&mut self, value: $T) {
                *self.get_mut() = value;
            }
            /// [`swap`](core::mem::swap) the values of two objects of the same type
            pub const fn swap(&mut self, other: &mut Self) {
                ::core::mem::swap(self.get_mut(), other.get_mut());
            }
            /// [`take`](core::mem::take) the inner value, leaving a default in its place
            #[inline]
            pub fn take(&mut self) -> $T
            where
                $T: ::core::default::Default,
            {
                ::core::mem::take(self.get_mut())
            }
            /// consumes the wrapper to create another with the given value
            #[inline]
            pub fn with<U>(self, value: U) -> $target<U> {
                $target::new(value)
            }
        }

        impl<$T> ::core::ops::Deref for $target<$T> {
            type Target = $T;

            fn deref(&self) -> &Self::Target {
                self.get()
            }
        }

        impl<$T> ::core::ops::DerefMut for $target<$T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.get_mut()
            }
        }
    };
}
