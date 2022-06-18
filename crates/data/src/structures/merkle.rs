pub struct MerkleTree<L> {
    pub leaves: Vec<L>,
}

pub trait MerkleSpecification {
    type Data;
    type Hash;
    type Leaf;
    type Node;

    fn generate(&self) -> Self;
    fn create_leaf(&self, data: Self::Data) -> Self::Leaf;
}

pub trait LeafSpec {
    type Data;
}