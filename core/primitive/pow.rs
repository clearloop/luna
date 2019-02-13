// pow
use std::cmp::Ordering;
use std::ops::Shl;
use num_bigint::{BigInt, Sign};
use super::utils::{hmac, hex};

/// How to control the mining time?
#[derive(Debug, Clone)]
pub struct ProofOfWork {
    pub barrel: Vec<u8>,
    pub target: BigInt
}

impl ProofOfWork {
    /// TODO - !optimize
    pub fn new(barrel: Vec<u8>, bits: i32) -> Self {
        let target = BigInt::from(1).shl(256 as usize - bits as usize);
        ProofOfWork { barrel: barrel, target: target }
    }
    
    fn prepare_data(&mut self, nonce: usize) -> Vec<u8> {
        let mut data = Vec::new();
        
        data.append(&mut self.barrel);
        data.append(&mut nonce.to_string().into_bytes());
        data
    }

    pub fn run(&mut self) -> (usize, Vec<u8>) {
        let mut hash_int: BigInt = BigInt::from(1);
        let mut nonce: usize = 0;

        while (nonce as i32) < i32::max_value() {
            let hash = hmac(self.clone().prepare_data(nonce));
            hash_int = BigInt::from_bytes_be(Sign::Plus, &hash);
            if hash_int.cmp(&self.target) == Ordering::Less {
                println!("\nnonce: {}", &nonce);
                println!("\nMining out block: 0x{}", hex(&hash));
                println!("\nReward 10 Ashes ~");
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, hash_int.to_bytes_be().1)
    }
}
