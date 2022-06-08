use crate::{primitives::{Clock, ObjectId, Transaction}, utils::timestamp};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum States {
    Invalid,
    Valid,
    Validating
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub id: ObjectId,
    pub hash: String,
    pub nonce: String,
    pub previous: String,
    pub state: States,
    pub timestamp: Clock,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(nonce: String, previous: String, transactions: Vec<Transaction>) -> Self {
        Self {
            id: ObjectId::new(),
            hash: String::from(""),
            nonce,
            previous,
            state: States::Valid,
            timestamp: timestamp(),
            transactions,
        }
    }
}


pub fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

pub fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining block...");
    let mut nonce = 0;

    loop {
        if nonce % 100000 == 0 {
            info!("nonce: {}", nonce);
        }
        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary_representation(&hash);
        if binary_hash.starts_with(crate::primitives::DIFFICULTY_PREFIX) {
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