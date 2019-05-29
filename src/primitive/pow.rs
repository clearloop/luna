// pow
use std::cmp::Ordering;
use std::ops::Shl;
use num_bigint::{BigInt, Sign};
use super::utils::hmac;

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

    pub fn run(&mut self) -> (usize, [u8; 32]) {
        let mut nonce: usize = 0;
        let mut hash_int: BigInt;
        let mut barrel_hash: [u8; 32] = hmac(self.clone().prepare_data(nonce));
        
        while (nonce as i32) < i32::max_value() {
            barrel_hash = hmac(self.clone().prepare_data(nonce));
            hash_int = BigInt::from_bytes_be(Sign::Plus, &barrel_hash);
            if hash_int.cmp(&self.target) == Ordering::Less {
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, barrel_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::ProofOfWork;
    #[test]
    fn pow() {
        let mut pow = ProofOfWork::new([0_u8; 8].to_vec(), 10);
        let (nonce, hash) = pow.run();
        assert_ne!(nonce, 0);
        assert_eq!(hash.len(), 32);
    }
}
