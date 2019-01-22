// Block
extern crate ed25519_dalek;
extern crate sha2;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    hash: [u8;32],
    timestamp: u64,
    prev_block_hash: [u8;32]
}

impl Block {
    pub fn genesis() -> Block {
        let mut hasher = Sha256::new();
        let mut default = Sha256::default();
        hasher.input(b"luna");
        default.input(b"default");

        let mut res = <[u8;32]>::default();
        let mut def = <[u8;32]>::default();
        res.copy_from_slice(&hasher.result()[..]);
        def.copy_from_slice(&default.result()[..]);

        Block {
            hash: res,
            timestamp: ts(),
            prev_block_hash: def
        }
    }
}

/// Luna uses millis as time unit.
fn ts() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Take your protein pills and put your helmet on.");

    // Time units: 1sec = 10^3ms
    return now.as_secs() * 1000 + now.subsec_millis() as u64;
}

fn main() {
    let b = Block::genesis();
    println!("{:?}", b);
}
