// Barrel
use std::fmt;
use std::ops::{Deref, DerefMut};
use crate::{bytes, partition};
use super::utils::{hmac, ts, hex};
use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Head {
    pub hash: [u8;32],
    pub nonce: usize
}

impl fmt::Debug for Head {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Head {{ \n  hash: 0x{}, \n  nonce: {}\n}}", hex(self.hash), self.nonce)
    }
}


#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Body {
    pub txs: Vec<u8>,
    pub pre_hash: [u8; 32],
    pub timestamp: u64
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(
            f,
            "Body {{ \n  txs: -, \n  pre_hash: 0x{}, \n  timestamp: {}\n}}",
            hex(self.pre_hash), self.timestamp
        )
    }
}

/// TODO: Miner Special Reward
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Barrel {
    pub head: Head,
    pub body: Body
}

impl Barrel {
    pub fn new<B>(msg: B, txs: Vec<u8>, pre_hash: [u8; 32]) -> Barrel
    where B: std::convert::AsRef<[u8]> {
        Barrel {
            head: Head {
                hash: hmac(msg), nonce: 0
            },
            body: Body {
                txs: txs,
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

#[cfg(test)]
mod tests {
    use super::Barrel;
    #[test]
    fn nonce() {
        let mut barrel = Barrel::new(
            b"halo, spaceboy", vec![], [0_u8;32]
        );

        barrel = barrel.nonce(1984_usize);
        assert_eq!(barrel.head.nonce, 1984);
    }
}
