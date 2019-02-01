// TX
// use std::ops::{Deref, DerefMut};
use crate::bytes;
use crate::utils::hmac;
use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize};

/// # TxInput
/// @id - H256
/// @signature - Signature
/// @pub_key - PublicKey
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TxInput {
    // no &'static, value does not live long enough.
    pub msg: Vec<u8>,
    pub pub_key: [u8; 32],
    pub signature: Vec<u8>
}

/// # TxOutput
/// @pub_key - PublicKey
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TxOutput {
    pub value: i32,
    pub pub_key: [u8; 32]
}

/// # Transaction
/// @vin - TxInput ; no Vec<>
/// @vout - TxOutput ; no Vec<>
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    pub vin: Vec<u8>,
    pub vout: Vec<u8>,
    pub txid: [u8;32]
}

impl Transaction {
    pub fn new(vin: Vec<u8>, vout: Vec<u8>) -> Self {
        let mut id = vec![];

        id.append(&mut vin.clone());
        id.append(&mut vout.clone());
        
        Transaction { txid: hmac(&id), vin: vin, vout: vout }
    }
}

bytes!(TxInput);
bytes!(TxOutput);
bytes!(Transaction);
