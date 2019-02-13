// Barrel
use crate::bytes;
use super::utils::{hmac, ts};
use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Barrel {
    pub head: Head,
    body: Body
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Head {
    pub hash: [u8;32],
    pub nonce: usize
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Body {
    pub txs: Vec<u8>,
    pub miner: [u8; 32],
    pub timestamp: u64
}

impl Barrel {
    pub fn new<B>(msg: B, txs: Vec<u8>, nonce: usize, miner: [u8; 32]) -> Barrel
    where B: std::convert::AsRef<[u8]> {
        Barrel {
            head: Head {
                hash: hmac(msg), nonce: nonce
            },
            body: Body {
                txs: txs,
                miner: miner,
                timestamp: ts()
            }
        }
    }
}

bytes!(Barrel);
