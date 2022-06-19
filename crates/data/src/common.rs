pub use types::*;


mod types {
    use bson;
    use chrono;

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
    pub type TimeStamp = bson::DateTime;

    pub type ObjectId = bson::oid::ObjectId;
}