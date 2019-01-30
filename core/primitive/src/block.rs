// Block
use crate::bytes;
use crate::utils::{hmac, ts};
use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Block {
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
    pub timestamp: u64
}

impl Block {
    pub fn new<B>(msg: B, txs: Vec<u8>, nonce: usize) -> Block
    where B: std::convert::AsRef<[u8]> {
        Block {
            head: Head {
                hash: hmac(msg),
                nonce: nonce
            },
            body: Body {
                txs: txs,
                timestamp: ts()
            }
        }
    }

    pub fn genesis() -> Self {
        Block::new(
            "Take your protein pill and put your helmet on.",
            vec![], 0
        )
    }
}

bytes!(Block);
