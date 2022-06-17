use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub enum Tokens {
    Auth(String)
}


#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct Token {
    kind: Tokens,
    token: String,
}

pub trait TokenSpecification {
    type Actor;
    type Client;
    type Context;
    type Data;

    fn create(&self) -> Self;
}