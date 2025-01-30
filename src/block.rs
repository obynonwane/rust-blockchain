use crate::{ProofOfWork, Transaction};
use serde::{Deserialize, Serialize};
use sled::IVec;

extern crate bincode;
use bincode::{deserialize, serialize};

struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };

        // Generate proof of work for the new block
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        // returns a tuple
        let (nonce, hash) = pow.run();
        // assigns the block hash & nonce
        block.nonce = nonce;
        block.hash = hash;

        return block;
    }

    // Accept a slice of bytes
    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

    // returns a vector of bytes
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> string {
        self.get_pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> str {
        self.hash.as_str()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    // return a collection (vector) of bytes - hashes of all the transation
    pub fn hash_transactions(&self) -> Vec<u8> {
        // declare an empty vector to hold transaction hashes
        let mut txhashs = vec![];

        // loop through each transaction in the transaction vector
        for transaction in &self.transactions {
            // add transaction id to the collection using extends method
            txhashs.extend(transaction.get_id);
        }

        // call sha256_digest from from the root crate to hash all transaction id
        crate::sha256_digest(txhashs.as_slice())
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = Vec![transaction.clone()];
        return Block::new_block(String::from("None"), transactions, 0);
    }
}

// Implementing the From trait for IVec: converts a block into an IVec
//
impl From<Block> for IVec {
    // from fn accepts a block
    fn from(b: Block) -> Self {
        // serialized a block into a binary byte array Vec<u8>
        let bytes = bincode::serialize(&b).unwrap();

        // wraps the serialized block into an IVec
        Self::from(bytes)
    }
}
