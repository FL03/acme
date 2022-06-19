use crate::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Name {
    pub complete: String,
    pub title: String,
    pub prefix: String,
    pub first: String,
    pub middle: String,
    pub last: String,
    pub suffix: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: ObjectId,
    pub key: String,
}

pub trait UserSpecification {
    type Identity;
    type Condition;
    type Context;

    fn authorize(&self) -> Self::Condition;
}