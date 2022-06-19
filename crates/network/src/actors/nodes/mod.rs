/*
    Author: Joe McCain III <jo3mccain@gmail.com> (https://pzzld.eth.link/)
    Module:

 */


pub enum Nodes {
    Full,
    Light
}

pub enum NodeStates {
    Computing,
    Controlling
}

pub trait NodeSpecification {
    type Appellation;
    type Client;
    type Configuration;
    type Data;

    fn activate(appellation: Self::Appellation) -> Self;
    fn configure(configuration: Self::Configuration) -> Self;
    fn connect(&mut self) -> Self::Client;
    fn destroy(&mut self) -> Self::Data;
}

#[derive(Clone, Debug)]
pub struct Node {
    pub appellation: String
}


pub mod utils {

}