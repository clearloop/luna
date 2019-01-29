// TX
use crate::utils::hmac;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    pub id: [u8;32],
    pub vin: Vec<u8>,
    pub vout: Vec<u8>
}

impl Transaction {
    fn is_coinbase(&self) -> bool {
        if &self.id == &[0_u8; 32] {
            true } else { false
        }
    }

    fn new(mut vin: Vec<u8>, mut vout: Vec<u8>) -> Self {
        let mut id = vec![];
        id.append(&mut vin);
        id.append(&mut vout);
        
        Transaction {
            id: hmac(&id),
            vin: vin,
            vout: vout
        }
    }
}

pub struct TxInput {
    pub id: [u8; 32],
    pub vout_idx: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>
}

pub struct TxOutput {
    pub value: i32,
    pub pub_key: Vec<u8>
}
