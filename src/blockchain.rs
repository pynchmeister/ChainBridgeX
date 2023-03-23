use crate::block::Block;
use crate::transaction::Transaction;
use crate::utils::generate_uuid;
use std::sync::{Arc, Mutex};

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub unconfirmed_transactions: Vec<Transaction>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            unconfirmed_transactions: Vec::new(),
            difficulty: 3,
        };

        let genesis_block = Block::new(
            0,
            chrono::Utc::now(),
            Vec::new(),
            generate_uuid(),
            0,
        );
        blockchain.blocks.push(genesis_block);

        blockchain
    }

    pub fn from_json(json: &str) -> Result<Blockchain, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        if transaction.is_valid() {
            self.unconfirmed_transactions.push(transaction);
            true
        } else {
            false
        }
    }

    pub fn mine_block(&mut self) -> Option<Block> {
        if self.unconfirmed_transactions.is_empty() {
            return None;
        }

        let previous_block = self.blocks.last().unwrap();
        let index = previous_block.index + 1;
        let timestamp = chrono::Utc::now();
        let transactions = self.unconfirmed_transactions.clone();
        let previous_hash = previous_block.hash.clone();

        let mut block = Block::new(index, timestamp, transactions, previous_hash, 0);

        let pow = crate::pow::ProofOfWork::new(self.difficulty);
        pow.run(&mut block);

        self.blocks.push(block.clone());
        self.unconfirmed_transactions.clear();

        Some(block)
    }

    pub fn is_valid(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if i > 0 && block.previous_hash != self.blocks[i - 1].hash {
                return false;
            }
            if !crate::pow::ProofOfWork::validate(&block, self.difficulty) {
                return false;
            }
        }
        true
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl ToString for Blockchain {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub type SharedBlockchain = Arc<Mutex<Blockchain>>;
