# Primitive

## refs

+ [Building a Multithreaded Web Server][1]



[1]: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html



---

# Primitive
> Basic Library

## barrel.rs
```rust
pub struct Head {
    pub hash: [u8;32],
    pub nonce: usize
}

pub struct Body {
    pub txs: Vec<u8>,
    pub pre_hash: [u8; 32],
    pub timestamp: u64
}

pub struct Barrel {
    pub head: Head,
    pub body: Body
}

impl Barrel {
    pub fn new<B: std::convert::AsRef<[u8]>>(
        msg: B, 
        txs: Vec<u8>, 
        pre_hash: [u8; 32]
    ) -> Barrel;
    pub fn nonce(mut self, nonce: usize) -> Self;
}

pub struct BarrelArray(Vec<Barrel>);
```
## ~~capsule.rs~~

## ~~control.rs~~

## cowboy.rs
```rust
use ed25519_dalek::{Keypair};
pub fn struct Cowboy(Keypair);
impl Cowboy {
    pub fn born() -> Self;
}
```

## io.rs
```rust
pub struct IO {
    path: PathBuf
}

impl IO {
    pub fn locate(path: &'static str) -> Self;
    pub fn clean(&self);
    pub fn push<B: std::convert::AsRef<[u8]>>(&self, data: B);
    pub fn pull(&self) -> Vec<u8>;
}
```

## pow.rs
```rust
pub struct ProofOfWork {
    pub barrel: Vec<u8>,
    pub target: BigInt
}

impl ProofOfWork {
    pub fn new(barrel: Vec<u8>, bits: i32) -> Self;
    pub fn run(&mut self) -> (usize, [u8;32]);
}
```

## tx.rs
```rust
pub struct TxInput {
    pub msg: Vec<u8>,
    pub from: [u8; 32],
    pub signature: Vec<u8>
}

impl TxInput {
    pub fn verify(&self, msg: &'static str, pub_key: [u8; 32]) -> bool;
}

pub struct TxOutput {
    pub value: i32,
    pub to: [u8; 32]
}

pub struct Transaction {
    pub vin: TxInput,
    pub vout: TxOutput,
    pub txid: [u8;32]
}

impl Transaction {
    pub fn new(vin: TxInput, vout: TxOutput) -> Self;
    pub fn reward(msg: &'static str, miner: [u8;32]) -> Self;
}

pub struct TransactionArray(Vec<Transaction>);
```

## utils.rs
```rust
pub fn ts() -> u64;
pub fn hex<B: std::convert::AsRef<[u8]>>(bytes: B) -> String;
pub fn hmac<B: std::convert::AsRef<[u8]>>(data: B) -> [u8;32];
```
