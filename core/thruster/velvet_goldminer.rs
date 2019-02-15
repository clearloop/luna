// Velvet Goldminer
use super::Cowboy;
use crate::primitive::{Barrel, ProofOfWork, Transaction, TransactionArray};

/// # Miner Flow Chart
/// -> Load Account
/// -> Sync Transaction Pool
/// -> Pack Transaction
/// -> Proof of Work
/// -> Pending Block
/// -> Boardcast
/// -> Blockchain Scale
/// -> Reward(vout)
pub trait Miner<T> {
    fn mine<B>(&self, msg: B, txs: TransactionArray, miner: [u8; 32]) -> Barrel
    where B: std::convert::AsRef<[u8]>;
    fn verify(&self, msg: &'static str, tx: Transaction, pub_key: [u8; 32]) -> bool;
    fn genesis(&self) -> Vec<u8>;
}

impl Miner<Cowboy> for Cowboy {
    fn mine<B>(&self, msg: B, txs: TransactionArray, pre_hash: [u8; 32]) -> Barrel
    where B: std::convert::AsRef<[u8]> {
        let barrel = Barrel::new(msg, txs.to_bytes(), pre_hash, self.public.to_bytes());
        let mut pow = ProofOfWork::new(barrel.to_bytes(), 10);

        let (nonce, _) = pow.run();
        barrel.nonce(nonce)
    }

    fn genesis(&self) -> Vec<u8> {
        Barrel::new(
            "Take your protein pills and put your helmet on.",
            vec![], [0_u8; 32], self.public.to_bytes()
        ).to_bytes()
    }

    fn verify(&self, msg: &'static str, tx: Transaction, pub_key: [u8; 32]) -> bool {
        let msg_s = String::from(msg).as_bytes().to_vec();

        tx.vin.verify(msg_s, pub_key)
    }
}
