struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

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
