// Guardian
use super::Cowboy;
use crate::primitive::{TxInput, TxOutput, Transaction};

/// Guardians of the galaxy
pub trait Guardian<T> {
    fn load(&self, value: i32, to: [u8; 32], msg: &'static str) -> Transaction;
}

impl Guardian<Cowboy> for Cowboy {
    fn load(&self, value: i32, to: [u8; 32], msg: &'static str) -> Transaction {
        let msg_s = String::from(msg).as_bytes().to_vec();

        let txin = TxInput {
            msg: msg_s.to_owned(),
            from: self.public.to_bytes(),
            signature: self.sign(&msg_s).to_bytes().to_vec(),
        };

        let txout = TxOutput {
            value: value,
            to: to
        };
        
        Transaction::new(txin, txout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sign_and_verify() {
        let cowboy = Cowboy::born();
        let tx = cowboy.load(1024, [0_u8;32], "halo, spaceboy");

        assert_eq!(tx.vin.verify("halo, spaceboy", cowboy.public.to_bytes()), true);
    }
}
