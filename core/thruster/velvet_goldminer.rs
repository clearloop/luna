// Velvet Goldminer
use super::Cowboy;
use crate::primitive::{Barrel, ProofOfWork, Transaction, TransactionArray};

pub trait Miner<T> {
    fn mine<B>(&self, msg: B, txs: TransactionArray, miner: [u8; 32]) -> Barrel
    where B: std::convert::AsRef<[u8]>;
}

impl Miner<Cowboy> for Cowboy {
    fn mine<B>(&self, msg: B, mut txs: TransactionArray, pre_hash: [u8; 32]) -> Barrel
    where B: std::convert::AsRef<[u8]> {
        txs.push(Transaction::reward("reward", self.public.to_bytes()));
        let barrel = Barrel::new(msg, txs.to_bytes(), pre_hash);
        let mut pow = ProofOfWork::new(barrel.to_bytes(), 10);

        let (nonce, _) = pow.run();
        barrel.nonce(nonce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn miner() {
        let cowboy = Cowboy::born();
        let barrel = cowboy.mine(
            "halo, spaceboy", TransactionArray::default(), [0_u8;32]
        );
        
        let txs = TransactionArray::from_bytes(barrel.body.txs);
        assert_eq!(txs.len(), 1);
        
        assert_eq!(txs[0].vin.msg, String::from("reward").as_bytes());
        assert_eq!(txs[0].vin.from, [0_u8;32]);
        assert_eq!(txs[0].vin.signature, []);
        
        assert_eq!(txs[0].vout.value, 10);
        assert_eq!(txs[0].vout.to, cowboy.public.to_bytes());
    }
}
