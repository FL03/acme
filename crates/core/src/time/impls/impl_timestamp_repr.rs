/*
    Appellation: impl_timestamp_repr <module>
    Created At: 2026.02.07:18:24:07
    Contrib: @FL03
*/
use crate::time::ts::Timestamp;
use crate::time::{Now, std_timestamp};

// impl Timestamp<u64> {
//     /// Returns the current timestamp as a [`Timestamp`] instance.
//     pub fn now() -> Self {
//         Self::new(std_timestamp())
//     }
// }

#[cfg(feature = "std")]
impl Now for Timestamp<u64> {
    type Output = Self;

    fn now() -> Self::Output {
        let ts = std_timestamp();
        Self::new(ts)
    }
}
#[cfg(feature = "chrono")]
impl Now for Timestamp<i64> {
    type Output = Self;

    fn now() -> Self::Output {
        let ts = chrono::Local::now().timestamp();
        Self::new(ts)
    }
}
