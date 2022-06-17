use super::Block;
use crate::BoxedError;

use serde::{Deserialize, Serialize};

pub trait Chains {
    type Account;
    type Block;
    type Configuration;

    fn activate(configuration: Self::Configuration) -> Self;
    fn client() -> Result<(), BoxedError>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    blocks: Vec<Block>,
}

impl Chains for Chain {
    type Account = ();
    type Block = ();
    type Configuration = ();

    fn activate(configuration: Self::Configuration) -> Self {
        todo!()
    }

    fn client() -> Result<(), BoxedError> {
        todo!()
    }
}
