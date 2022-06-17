

// Create an account specification
pub trait AccountSpecs {
    type Address;
    type Chain;
    type Context;
    type Data;

    fn authenticate(&self) -> Self;
    fn create(&self, chain: Self::Chain, context: Self::Context, data: Self::Data) -> Self;
    fn describe(&self, context: Self::Context) -> Self::Data;
}