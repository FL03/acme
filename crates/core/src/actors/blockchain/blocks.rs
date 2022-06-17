/*



    Module Requirements:
        - Describe the operations that may be applied to a block
        -
 */
use crate::{BlockData, BlockHash, BlockId, BlockNonce, TimeStamp};
use crate::utils::timestamp;
use serde::{Deserialize, Serialize};

// TODO - Finish implementing the block specification
pub trait BlockSpec {
    type BlockId;
    type BlockHash;
    type Data;
    type Hash;
    type Nonce;
    type Timestamp;

    fn calculate_hash(&self) -> Vec<u8>;
    fn create(&self, data: Self::Data, nonce: Self::Nonce, previous: Self::Hash) -> Self;
    fn describe(&self);

    fn fetch(&self, id: Self::BlockId) -> Self;
}

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct Block {
    pub id: BlockId,
    pub data: BlockData,
    pub hash: BlockHash,
    pub nonce: BlockNonce,
    pub previous: BlockHash,
    pub timestamp: TimeStamp,
}

impl Block {
    pub fn new(data: BlockData, nonce: BlockNonce, previous: BlockHash) -> Self {
        let id = BlockId::new();
        let hash: BlockHash = "".to_string();
        let timestamp = timestamp();

        Self { id, data, hash, nonce, previous, timestamp }
    }
}
