// TX
use std::ops::{Deref, DerefMut};
use crate::{bytes, deref};
use crate::utils::hmac;
use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize};

/// # Transaction
/// @vin - TxInput  ; no Vec<>
/// @vout - TxOutput ; no Vec<>
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    pub id: [u8;32],
    pub vin: Vec<u8>,
    pub vout: Vec<u8>
}

/// # TxInput
/// @id - H256
/// @signature - Signature
/// @pub_key - PublicKey
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TxInput {
    pub id: [u8; 32],
    pub vout_idx: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>
}

/// # TxOutput
/// @pub_key - PublicKey
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TxOutput {
    pub value: i32,
    pub pub_key: Vec<u8>
}

bytes!(TxInput);
bytes!(TxOutput);
bytes!(Transaction);
