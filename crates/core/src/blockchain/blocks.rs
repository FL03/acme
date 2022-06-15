/*



    Module Requirements:
        - Describe the operations that may be applied to a block
        -
 */
use crate::{
    types::{BlockData, BlockHash, BlockId, BlockNonce, LocalTime, TimeStamp},
    utils::timestamp,
};
use serde::{Deserialize, Serialize};

pub use utils::*;

// TODO - Finish implementing the block specification
pub trait BlockSpec {
    type BlockId;
    type BlockHash;
    type Data;
    type Hash;
    type Nonce;
    type Timestamp;

    fn compute(&self) -> Vec<u8>;
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

pub mod utils {
    use log::info;
    use serde_json::json;
    use sha2::{Digest, Sha256};

    use crate::{DIFFICULTY_PREFIX, BlockData, BlockHash, BlockId, BlockNonce, TimeStamp};

    // Calculate the hash of a Block using standard Block parameters
    pub fn calculate_hash(id: BlockId, data: BlockData, nonce: BlockNonce, previous: BlockHash, timestamp: TimeStamp) -> Vec<u8> {
        let data = json!({
            "id": id,
            "data": data,
            "nonce": nonce,
            "previous": previous,
            "timestamp": timestamp
        });
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }

    // Represent a Block hash in the proper format for the binary interface
    pub fn hash_to_binary_representation(hash: &[u8]) -> String {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res
    }

    // Defines the standard method in which blocks are to be mined
    pub fn mine_block(id: BlockId, data: BlockData, nonce: BlockNonce, previous: BlockHash, timestamp: TimeStamp) -> (u64, String) {
        info!("mining block...");
        let mut nonce = 0;

        loop {
            if nonce % 100000 == 0 {
                info!("nonce: {}", nonce);
            }
            let hash = calculate_hash(id, data.clone(), nonce, previous.clone(), timestamp);
            let binary_hash = hash_to_binary_representation(&hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                info!(
                    "mined! nonce: {}, hash: {}, binary hash: {}",
                    nonce,
                    hex::encode(&hash),
                    binary_hash
                );
                return (nonce, hex::encode(hash));
            }
            nonce += 1;
        }
    }
}