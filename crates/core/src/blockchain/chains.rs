use crate::{
    blockchain::Block,
    types::BoxedError,
};
use serde::{Deserialize, Serialize};

pub trait ChainSpec {
    type Block;
    type Configuration;

    fn activate(configuration: Self::Configuration) -> Self;
    fn connect() -> Result<(), BoxedError>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    blocks: Vec<Block>,
}
