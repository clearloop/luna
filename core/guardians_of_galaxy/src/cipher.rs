extern crate primitive;
use primitive::account::Account;
use primitive::tx::{Transaction, TxInput, TxOutput};

/// # Transfer
/// wrap account directly, keep some attr space to construct better in future.
#[derive(Debug)]
pub struct Cowboy {
    account: Account
}

impl Cowboy {
    pub fn new() -> Self { Cowboy { account: Account::new()}}

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
}