// Guardian
use super::Cowboy;
use crate::primitive::{TxInput, TxOutput, Transaction};

/// # Transfer
/// wrap account directly, keep some attr space to construct better in future.
pub trait Guardian<T> {
    fn load(&self, value: i32, to: [u8; 32], msg: &'static str) -> Transaction;
}

impl Guardian<Cowboy> for Cowboy {
    fn load(&self, value: i32, to: [u8; 32], msg: &'static str) -> Transaction {
        let msg_s = String::from(msg).as_bytes().to_vec();

        let txin = TxInput {
            msg: msg_s.to_owned(),
            pub_key: self.public.to_bytes(),
            signature: self.sign(&msg_s).to_bytes().to_vec(),
        };

        let txout = TxOutput {
            value: value,
            pub_key: to
        };

        Transaction::new(txin.to_bytes(), txout.to_bytes())
    }
}
