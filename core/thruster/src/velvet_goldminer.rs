// Velvet Goldminer
use crate::Cowboy;
use primitive::tx::{Transaction, TxInput};
use primitive::pow::ProofOfWork;
use primitive::barrel::Barrel;

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
    fn mine<B>(&self, msg: B, txs: Vec<u8>, nonce: usize, miner: [u8; 32]) -> (usize, Vec<u8>)
        where B: std::convert::AsRef<[u8]>;
    fn verify(&self, msg: &'static str, tx: Transaction) -> bool;
}

impl Miner<Cowboy> for Cowboy {
    fn mine<B>(&self, msg: B, txs: Vec<u8>, nonce: usize, miner: [u8; 32]) -> (usize, Vec<u8>)
    where B: std::convert::AsRef<[u8]> {
        let barrel = Barrel::new(msg, txs, nonce, miner).to_bytes();
        // 10 bits - difficulty of mining
        let mut pow = ProofOfWork::new(barrel, 10);

        pow.run()
    }
    
    fn verify(&self, msg: &'static str, tx: Transaction) -> bool {
        let input = TxInput::from_bytes(tx.vin);
        let msg_s = String::from(msg).as_bytes().to_vec();

        input.verify(msg_s)
    }
}
