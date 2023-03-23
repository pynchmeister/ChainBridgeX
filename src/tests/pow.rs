use rand::{self, Rng};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct ProofOfWork {
    difficulty: u64,
}

impl ProofOfWork {
    pub fn new(difficulty: u64) -> Self {
        Self { difficulty }
    }

    pub fn run(&self, data: &[u8]) -> (u64, Vec<u8>) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        let mut rng = rand::thread_rng();

        loop {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.update(&nonce.to_le_bytes());

            let result = hasher.finalize();

            if check_difficulty(&result, self.difficulty) {
                hash = result.to_vec();
                break;
            }

            nonce = rng.gen();
        }

        (nonce, hash)
    }
}

fn check_difficulty(hash: &[u8], difficulty: u64) -> bool {
    let mut leading_zeros = 0;

    for byte in hash.iter() {
        if *byte != 0 {
            break;
        }

        leading_zeros += 1;
    }

    leading_zeros >= difficulty as usize
}

