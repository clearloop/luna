// Velvet Goldminer
pub use primitive::cowboy::Cowboy;
use primitive::tx::{Transaction, TxInput};

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
    fn verify(smg: &'static str, tx: Transaction) -> bool;
}

impl Miner<Cowboy> for Cowboy {
    fn verify(msg: &'static str, tx: Transaction) -> bool {
        let msg_s = String::from(msg).as_bytes().to_vec();
        let input = TxInput::from_bytes(tx.vin);

        input.verify(msg_s)
    }
}
