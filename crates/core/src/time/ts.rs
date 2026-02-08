/*
    Appellation: ts <module>
    Created At: 2026.02.07:18:19:53
    Contrib: @FL03
*/

/// [`RawTimestamp`] is a marker trait used to define the _types_ that can be used as the inner
/// type of a [`Timestamp`].
pub trait RawTimestamp {
    private! {}
}
/// [`IntoTimestamp`] is a trait used to convert a type into a [`Timestamp`] instance,
/// consuming the caller in the process.
pub trait IntoTimestamp<T> {
    fn into_timestamp(self) -> Timestamp<T>;
}

/// The [`Timestamp`] is a generic implementation of a timestamp, used to define a single point
/// in time.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Timestamp<T = u64>(pub T);

/*
 ************* Implementations *************
*/

impl<U, T> IntoTimestamp<T> for U
where
    T: RawTimestamp,
    Timestamp<T>: From<U>,
{
    fn into_timestamp(self) -> Timestamp<T> {
        Timestamp::from(self)
    }
}

macro_rules! raw_timestamp {
    ($($T:ty),* $(,)?) => {
        $(raw_timestamp! { @impl $T })*
    };
    (@impl $T:ty) => {
        impl RawTimestamp for $T {
            seal! {}
        }
    };
}

raw_timestamp! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
}
