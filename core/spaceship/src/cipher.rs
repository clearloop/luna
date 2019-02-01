extern crate primitive;
use primitive::account::Account;
use primitive::tx::{Transaction, TxInput, TxOutput};
// use primitive::utils::hmac;

pub enum Error {
    Custom(String)
}

/// # Transfer
/// wrap account directly, keep some attr space to construct better in future.
#[derive(Debug)]
pub struct Transfer {
    account: Account
}

impl Transfer {
    pub fn new() -> Self { Transfer { account: Account::new()}}

    pub fn generate(&self, value: i32, to: [u8; 32], msg: &'static str) -> Transaction {
        let msg_s = String::from(msg).as_bytes().to_vec();

        let txin = TxInput {
            msg: msg_s.to_owned(),
            pub_key: self.account.public.to_bytes(),
            signature: self.account.sign(&msg_s).to_bytes().to_vec(),
        };
        
        let txout = TxOutput {
            value: value,
            pub_key: to
        };

        Transaction::new(txin.to_bytes(), txout.to_bytes())
    }

    pub fn verify(&self, msg: &'static str, tx: Transaction) -> bool {
        let msg_s = String::from(msg).as_bytes().to_vec();

        self.account.verify(
            msg_s, TxInput::from_bytes(tx.vin).signature
        )
    }
}
