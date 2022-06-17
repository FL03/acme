/*
    Module: Actors
    Overview:
        Definition: Actor
            In computer programming, actors are defined as mathematical models of computation.
        Goals:
            - Generally describe the fundamental actors that are being leveraged throughout
 */

pub mod blockchain;

pub use blockchain::*;

mod loggers;

pub use loggers::*;

pub trait Actor {
    type Identity;
    type Classification;
    type Computation;
    type Determinate;
    type State;

    fn client(&self, identity: Self::Identity) -> Self;
    fn compute(&self, computation: Self::Computation) -> Self::Determinate;
    fn determine(&self, req: String) -> Self::Determinate;
}

