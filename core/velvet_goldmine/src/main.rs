/// Miner Flow Chart
/// -> Load Account
/// -> Sync Transaction Pool
/// -> Pack Transaction
/// -> Proof of Work
/// -> Pending Block
/// -> Boardcast
/// -> Blockchain Scale
//! -> Reward(vout)

pub fn verify(msg: &'static str, tx: Transaction) -> bool {
    let msg_s = String::from(msg).as_bytes().to_vec();

    self.account.verify(
        msg_s, TxInput::from_bytes(tx.vin).signature
    )
}

fn main() {
    println!("Hello, world!");
}
