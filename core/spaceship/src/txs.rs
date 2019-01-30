extern crate primitive;
pub use primitive::tx::{Transaction, TxInput, TxOutput};
use primitive::account::Account;
use primitive::utils::hmac;

pub trait Symmetric {
    fn new(mut input: Vec<u8>, mut output: Vec<u8>, account: Vec<u8>) -> Self;
    fn verify(&self);
}

impl Symmetric for Transaction {
    fn new(mut input: Vec<u8>, mut output: Vec<u8>, account: Vec<u8>) -> Self {
        let mut id = vec![];

        id.append(&mut input);
        id.append(&mut output);
        
        Transaction { id: hmac(&id), vin: input, vout: output }
    }

    fn verify(&self) {
        println!("Verify");
    }
}
