pub use bson::oid::ObjectId;

pub use blockchain::*;
pub use dates::*;
pub use errors::*;


mod blockchain {
    pub const DIFFICULTY_PREFIX: &str = "00";

    pub type BlockData = String;
    pub type BlockId = super::ObjectId;
    pub type BlockHash = String;
    pub type BlockNonce = u64;
}

mod dates {
    use bson;
    use chrono;

    pub enum Dates {
        Datetime(chrono::DateTime<chrono::Local>),
        Localtime(chrono::Local),
        Timestamp(bson::DateTime)
    }

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
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
}

mod errors {
    use std::error::Error;

    pub enum Errors {
        Default(BoxedError)
    }
    pub type BoxedError = Box<dyn Error>;

}