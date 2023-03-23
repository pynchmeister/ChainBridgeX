use crate::transaction::Transaction;
use chrono::{DateTime, Utc};

pub struct Block {
    pub index: u32,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: DateTime<Utc>,
        transactions: Vec<Transaction>,
        previous_hash: String,
        nonce: u32,
    ) -> Block {
        let hash = calculate_hash(index, &timestamp, &transactions, &previous_hash, nonce);
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn from_json(json: &str) -> Result<Block, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

fn calculate_hash(
    index: u32,
    timestamp: &DateTime<Utc>,
    transactions: &Vec<Transaction>,
    previous_hash: &String,
    nonce: u32,
) -> String {
    use sha2::{Digest, Sha256};

    let mut data = vec![
        index.to_string(),
        timestamp.to_rfc3339(),
        previous_hash.clone(),
        nonce.to_string(),
    ];

    for transaction in transactions {
        data.push(transaction.to_string());
    }

    let data_str = data.join("");
    let mut hasher = Sha256::new();
    hasher.update(data_str.as_bytes());
    let result = hasher.finalize();

    hex::encode(result)
}
