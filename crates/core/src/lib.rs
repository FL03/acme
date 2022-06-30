/*
    Appellation: acme-core
    Context: library
    Creator:
    Description:
        Core feature library for acme, an all-in-one blockchain toolkit for building optimized
        EVM compatible apps and chains.
 */
mod actors;
mod apps;
mod configure;
mod connect;
mod contexts;
mod controllers;
mod errors;
mod utils;

pub use crate::{
    actors::*,
    apps::*,
    common::*,
    configure::*,
    connect::*,
    controllers::*,
    errors::*,
    utils::*,
};

mod common {
    pub use types::*;

    mod types {
        pub use bson::DateTime;
        pub use bson::oid::ObjectId;
        pub use chrono::{Local, Utc};

        pub type Container<T = String> = Dictionary<Vec<T>>;
        pub type Dictionary<T = String> = std::collections::HashMap<String, T>;

        #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct Timestamp {
            timestamp: i64,
        }

        impl Timestamp {
            pub fn new(timestamp: i64) -> Self {
                Self { timestamp }
            }
            pub fn utc() -> Self {
                Self::new(Utc::now().timestamp())
            }
            pub fn local() -> Self {
                Self::new(Local::now().timestamp())
            }
        }
    }
}