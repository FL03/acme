pub mod logger;

pub use logger::*;

pub enum Loggers {
    Debug,
    Info,
    Tracing,