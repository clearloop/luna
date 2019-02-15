// TX
use std::fmt;
use std::ops::{Deref, DerefMut};
use crate::{bytes, partition};
use super::utils::{hmac, hex};
use bincode::{serialize, deserialize};
use ed25519_dalek::{PublicKey, Signature};
use serde_derive::{Serialize, Deserialize};

/// # TxInput
/// @id - H256
/// @signature - Signature
/// @pub_key - PublicKey
#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TxInput {
    // no &'static, value does not live long enough.
    pub msg: Vec<u8>,
    pub pub_key: [u8; 32],
    pub signature: Vec<u8>
}

impl TxInput {
    pub fn verify(&self, msg: Vec<u8>, pub_key: [u8; 32]) -> bool {
        let public = PublicKey::from_bytes(&pub_key).unwrap();
        public.verify(
            &msg, &Signature::from_bytes(&self.signature).unwrap()
        ).is_ok()
    }
}

impl fmt::Debug for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "TxInput {{ \n  msg: {:?}, \n  pub_key: 0x{}, \nsignature: {}\n}}",
               String::from_utf8(self.msg.to_owned()), hex(self.pub_key), hex(&self.signature))
    }
}

/// # TxOutput
/// @pub_key - PublicKey
#[derive(Serialize, Clone, Deserialize, Default, PartialEq)]
pub struct TxOutput {
    pub value: i32,
    pub pub_key: [u8; 32]
}

impl fmt::Debug for TxOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "TxOutput {{ \n  value: {}, \n  pub_key: {}\n}}",
               self.value, hex(self.pub_key))
    }
}

/// # Transaction
/// @vin - TxInput ; no Vec<>
/// @vout - TxOutput ; no Vec<>
#[derive(Serialize, Debug, Deserialize, Default, PartialEq)]
pub struct Transaction {
    pub vin: TxInput,
    pub vout: TxOutput,
    pub txid: [u8;32]
}

impl Transaction {
    pub fn new(vin: TxInput, vout: TxOutput) -> Self {
        let mut id = vec![];

        id.append(&mut vin.clone().to_bytes());
        id.append(&mut vout.clone().to_bytes());

        Transaction { txid: hmac(&id), vin: vin, vout: vout }
    }
}

bytes!(TxInput);
bytes!(TxOutput);
bytes!(Transaction);
partition!(Transaction, TransactionArray);

#[cfg(test)]
mod tests {
    use super::{TxInput, TxOutput, Transaction, TransactionArray};
    #[test]
    fn bytes_convert() {
        let vin = TxInput {
            msg: "halo, spaceboy".as_bytes().to_vec(),
            pub_key: [0_u8;32],
            signature: vec![0_u8;64]
        };

        let vout = TxOutput {
            value: 10,
            pub_key: [0_u8;32]
        };

        let tx = Transaction::new(vin, vout);
        let mut txs = TransactionArray::default();
        txs.push(tx);
        
        let bytes = txs.to_bytes();
        let txsfb = TransactionArray::from_bytes(bytes);
        assert_eq!(txs, txsfb);
    }    
}
