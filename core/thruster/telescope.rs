// Telescope
use super::Cowboy;
use crate::primitive::{BarrelChain, TransactionArray};
    
pub trait Telescope {
    fn utxo(&self, address: [u8; 32], barrel_chain: BarrelChain) -> usize;
}

impl Telescope for Cowboy {
    fn utxo(&self, _address: [u8; 32], barrel_chain: BarrelChain) -> usize {
        let mut sum: usize = 0;

        for barrel in barrel_chain.iter() {
            let txs = TransactionArray::from_bytes(barrel.to_owned().body.txs);

            for tx in txs.iter() {
                sum += tx.vout.value as usize
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Revolver;
    #[test]
    fn utxo() {
        let cowboy = Cowboy::born();
        let revolver = Revolver::locate(
            "test_utxo_pool", "test_utxo_chain", "test_utxo_chain"
        );
        
        assert_eq!(cowboy.utxo(cowboy.public.to_bytes(), revolver.chain), 10);
    }
}
