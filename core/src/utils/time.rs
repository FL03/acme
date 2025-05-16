/*
  Appellation: time <module>
  Contrib: @FL03
*/


/// [systime] is a method that returns the current time, in milliseconds, using the 
/// [`SystemTime`](std::time::SystemTime) API.
#[cfg(feature = "std")]
#[inline]
pub fn systime() -> u128 {
  std::time::SystemTime:: now()
    .duration_since(std:: time:: UNIX_EPOCH)
    .map(|d| d.as_millis())
    .expect("time went backwards")
}