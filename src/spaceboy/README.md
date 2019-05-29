# Spaceboy
> in the flesh?

## mod.rs
```rust
pub struct Spaceboy {
    master: Cowboy,
    revolver: Revolver
}

impl Spaceboy {
    pub fn new(pool: &'static str, chain: &'static str, cowboy: &'static str) -> Spaceboy;
    pub fn hello() -> Spaceboy;
    pub fn watch(&mut self, account: [u8;32]) -> usize;
    pub fn balance(&mut self) -> usize;
    pub fn transfer(&mut self, value: i32, to: [u8; 32], msg: &'static str);
    pub fn pack(&mut self, msg: &'static str);
}
```
