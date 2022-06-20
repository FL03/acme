/*
    Library: acme-core
    Version: 0.1.5

    Overview

 */
mod actors;
mod controllers;
mod utils;

pub use bson::oid::ObjectId;

pub use actors::*;
pub use common::*;
pub use controllers::*;
pub use utils::*;


mod common {
    use bson;
    use chrono;

    pub enum Dates {
        Datetime(chrono::DateTime<chrono::Local>),
        Localtime(chrono::Local),
        Timestamp(bson::DateTime),
    }

    pub type DateTime = chrono::DateTime<LocalTime>;
    pub type LocalTime = chrono::Local;
    pub type TimeStamp = bson::DateTime;
}

pub mod errors {
    pub use config::ConfigError;
    use std::error::Error;

    pub enum Errors {
        Default(BoxedError)
    }

    pub type AsyncError = Box<dyn Error + Send + Sync + 'static>;
    pub type BoxedError = Box<dyn Error>;
}
