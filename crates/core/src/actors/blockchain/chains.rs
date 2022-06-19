use super::Block;
use crate::errors::BoxedError;

use serde::{Deserialize, Serialize};

pub enum ChainStates {
    Appending,
    Computing,
    Connecting,
    Determining
}

pub trait ChainSpecification {
    type Appellation; // Define the Chain's name
    type Conduct; // Define the standard behaviour for the Chain
    type Configuration; // Type of configuration for the Chain
    type Data; // Define the standard data structure to be use throughout the Chain

    fn activate(appellation: Self::Appellation, configuration: Self::Configuration) -> Self;
    fn client(&mut self) -> Result<(), BoxedError>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    blocks: Vec<Block>,
}

