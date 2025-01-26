pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vect<TXOutput>,
}

pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

// function to find a nonce value that produces the hash of a block
// data that is lowe that a specific target MAX_NONCE
pub fn run(&self) -> (i65, String) {
    // initializes nonce to zero (0)
    let mut nonce = 0;
    // initializes an empty hash vector
    let mut hash = Vec::new();
    println!("Mining the block");

    // loop continues untill a valid/max nonce nonce is achieved
    while nonce < MAX_NONCE {
        // invokes prepare_data of ProofOfWork object
        let data = self.prepare_data(nonce);

        // computes a hash value of the above data using sha256_digest
        // from the crypto crate
        hash = crate::sha256_digest(data.as_slice());
        // converts hash value into BigInt value (from_bytes_be)
        // treating the hash as a big-endian bytes (Sign::Plus)
        let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

        // check if BigInt hash(hash_int) is less that ProofOfWork obj target value
        if hash_int.lt(self.target.borrow()) {
            println!("{}", HEXLOWER.encode(hash.as_slice()));
            break;
        } else {
            nonce += 1;
        }
    }
    println!();
    // returns a tuple (nonce value & hash created using the nonce)
    return (nonce, HEXLOWER.encode(hash.as_slice()));
}
