/*
    Appellation: impl_timestamp_repr <module>
    Created At: 2026.02.07:18:24:07
    Contrib: @FL03
*/
use crate::time::Now;
use crate::time::ts::Timestamp;

#[cfg(feature = "std")]
impl Now for Timestamp<u64> {
    type Output = Self;

    fn now() -> Self::Output {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Self::new(ts)
    }
}

impl Now for Timestamp<u128> {
    type Output = Self;

    fn now() -> Self::Output {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
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
