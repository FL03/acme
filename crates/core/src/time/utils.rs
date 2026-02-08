/*
    Appellation: utils <module>
    Created At: 2026.02.07:18:20:40
    Contrib: @FL03
*/

#[cfg(feature = "std")]
/// returns the current timestamp as a `u64` representing the number of seconds since the UNIX epoch.
pub fn std_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[cfg(all(feature = "chrono", feature = "std"))]
pub fn chrono_timestamp() -> i64 {
    chrono::Local::now().timestamp()
}
