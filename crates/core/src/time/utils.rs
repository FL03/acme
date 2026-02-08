/*
    Appellation: utils <module>
    Created At: 2026.02.07:18:20:40
    Contrib: @FL03
*/

#[cfg(feature = "std")]
pub(crate) fn std_epoch() -> std::time::Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
}

#[cfg(feature = "std")]
/// returns the current timestamp as a `u64` representing the number of seconds since the UNIX epoch.
pub fn stdtime_secs() -> u64 {
    std_epoch().as_secs()
}
