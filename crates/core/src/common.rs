use bson;
use chrono;
pub use blockchain::*;

pub type BoxedError = Box<dyn std::error::Error>;

pub type DateTime = chrono::DateTime<LocalTime>;
pub type LocalTime = chrono::Local;
pub type ObjectId = bson::oid::ObjectId;
pub type TimeStamp = bson::DateTime;

pub struct Timestamp {
    pub datetime: chrono::DateTime<chrono::Local>,
    pub timestamp: bson::DateTime,
}

impl Timestamp {
    pub fn create(&self) -> Self {
        let datetime = chrono::Local::now();
        Self {
            datetime: datetime.clone(),
            timestamp: datetime.into(),
        }
    }
}


mod blockchain {
    pub const DIFFICULTY_PREFIX: &str = "00";

    pub type BlockData = String;
    pub type BlockId = super::ObjectId;
    pub type BlockHash = String;
    pub type BlockNonce = u64;
}