# Thruster
> Tool Library

## guardian.rs
```rust
pub trait Guardian<T> {
    fn load(&self, value: i32, to: [u8; 32], msg: &'static str) -> Transaction;
}
```

## revolver.rs
```rust
struct Wheel {
    pub pool:  IO,
    pub chain: IO,
    pub cowboy: IO
}

pub struct Revolver {
    wheel: Wheel,
    pub pool:  TransactionArray,
    pub chain: BarrelChain
}

impl Revolver {
    pub fn locate(p: &'static str, c: &'static str, cb: &'static str) -> Self;
    pub fn master(&self) -> Cowboy;
    pub fn shoot(&mut self, tx: Transaction);
    pub fn revolve(&mut self, bc: Barrel);
    pub fn explode(&self);
}
```

## telescope.rs
```rust
pub trait Telescope {
    fn utxo(&self, address: [u8; 32], barrel_chain: BarrelChain) -> usize;
}
```

## miner.rs
```rust
pub trait Miner<T> {
    fn mine<B>(&self, msg: B, txs: TransactionArray, miner: [u8; 32]) -> Barrel
    where B: std::convert::AsRef<[u8]>;
}
```
