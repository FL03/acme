/*
    Appellation: now <module>
    Created At: 2026.02.07:18:22:57
    Contrib: @FL03
*/

pub trait Now {
    type Output;

    fn now() -> Self::Output;
}
