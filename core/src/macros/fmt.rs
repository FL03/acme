/*
    Appellation: fmt <module>
    Contrib: @FL03
*/

macro_rules! fmt_wrapper {
    ($s:ident<$T:ident>($($trait:ident($($fmt:tt)*)),* $(,)?)) => {
        $(
            fmt_wrapper!(@impl $s::<$T>::$trait($($fmt)*));
        )*
    };
    (@impl $s:ident::<$T:ident>::$trait:ident($($fmt:tt)*)) => {
        impl<$T> ::core::fmt::$trait for $s<$T>
        where
        $T: ::core::fmt::$trait
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $($fmt)*, self.0)
            }
        }
    };
}