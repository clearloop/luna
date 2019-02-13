// Barrel
use crate::{bytes, partition};
use super::utils::{hmac, ts};
use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Barrel {
    pub head: Head,
    pub body: Body
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
    pub pre_hash: [u8; 32],
    pub timestamp: u64
}

impl Barrel {
    pub fn new<B>(msg: B, txs: Vec<u8>, pre_hash: [u8; 32], miner: [u8; 32]) -> Barrel
    where B: std::convert::AsRef<[u8]> {
        Barrel {
            head: Head {
                hash: hmac(msg), nonce: 0
            },
            body: Body {
                txs: txs,
                miner: miner,
                pre_hash: pre_hash,
                timestamp: ts()
            }
        }
    }

    pub fn nonce(mut self, nonce: usize) -> Self {
        self.head.nonce = nonce;
        self
    }
}

bytes!(Barrel);
partition!(Barrel, BarrelChain);
