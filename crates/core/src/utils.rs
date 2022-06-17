use crate::{LocalTime, TimeStamp};

pub use crate::actors::blockchain::utils::*;

pub fn timestamp() -> TimeStamp {
    let current_time: TimeStamp = LocalTime::now().into();
    return current_time;
}