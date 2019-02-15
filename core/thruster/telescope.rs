// Telescope
use super::Cowboy;
use crate::primitive::{BarrelChain};//, TransactionArray};
    
pub trait Telescope {
    fn utxo(&self, address: [u8; 32], barrel_chain: BarrelChain) -> usize;
}

impl Telescope for Cowboy {
    fn utxo(&self, _address: [u8; 32], barrel_chain: BarrelChain) -> usize {
        let mut sum: usize = 0;

        for barrel in barrel_chain.iter() {
            if barrel.body.txs.len() == 0 { continue }
            println!("{:?}", barrel.body.txs);
            sum += 1;
            break;
        }

        sum
    }
}
