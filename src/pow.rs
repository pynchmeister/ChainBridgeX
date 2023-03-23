use sha3::{Digest, Sha3_256};

pub struct ProofOfWork {
    difficulty: u64,
}

impl ProofOfWork {
    pub fn new(difficulty: u64) -> ProofOfWork {
        ProofOfWork { difficulty }
    }

    pub fn run(&self, data: &[u8]) -> u64 {
        let mut nonce: u64 = 0;
        let mut hash_output = [0; 32];

        loop {
            let input = [data, &nonce.to_le_bytes()].concat();
            let hash = Sha3_256::digest(&input);

            if hash[..8].eq(&[0u8; 8]) {
                hash_output.copy_from_slice(&hash[..]);
                break;
            } else {
                nonce += 1;
            }
        }

        nonce
    }

    pub fn validate(&self, data: &[u8], nonce: u64) -> bool {
        let input = [data, &nonce.to_le_bytes()].concat();
        let hash = Sha3_256::digest(&input);

        hash[..8].eq(&[0u8; 8])
    }
}

