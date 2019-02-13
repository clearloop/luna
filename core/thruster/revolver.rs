// Revolver
use super::Cowboy;
use crate::primitive::{IO, Transaction, TransactionArray, Barrel, BarrelChain};

///IO Efficiency
///TODO: files' size, just like, 00, 01...
pub struct Revolver {
    pool:  IO,
    chain: IO,
    cowboy: IO
}

impl Revolver {
    pub fn locate() -> Self {
        let home = IO::home_dir();
        let _ = IO::create_dir(&home.join(".luna"));
        let luna = &home.join(".luna");

        Revolver {
            pool:  IO::locate(&luna.join("pool")),
            chain: IO::locate(&luna.join("chain")),
            cowboy: IO::locate(&luna.join("cowboy"))
        }
    }

    pub fn master(&self) -> std::io::Result<Cowboy> {
        if &self.cowboy.exists() == &false {
            &self.cowboy.push(Cowboy::born().to_bytes())?;
        }

        Ok(Cowboy::from_bytes(self.cowboy.pull()?))
    }

    pub fn push_to_pool(&self, bytes: Vec<u8>) -> std::io::Result<()> {
        let pool_bytes = self.pool.pull();
        let mut txs: TransactionArray;
        match pool_bytes.is_ok() {
            true => txs = TransactionArray::from_bytes(pool_bytes.unwrap()),
            false => txs = TransactionArray::default()
        }

        txs.push(Transaction::from_bytes(bytes));
        self.pool.push(txs.to_bytes())
    }

    pub fn scan_pool(&self) -> std::io::Result<(TransactionArray)> {
        Ok(TransactionArray::from_bytes(self.pool.pull()?))
    }

    pub fn empty_pool(&self) -> std::io::Result<()> {
        self.pool.clean()
    }

    pub fn stretch_chain(&self, bytes: Vec<u8>) -> std::io::Result<()> {
        let chain_bytes = self.chain.pull();
        let mut bc: BarrelChain;
        match chain_bytes.is_ok() {
            true => bc = BarrelChain::from_bytes(chain_bytes.unwrap()),
            false => bc = BarrelChain::default()
        }

        bc.push(Barrel::from_bytes(bytes));
        self.chain.push(bc.to_bytes())
    }

    pub fn scan_chain(&self) -> std::io::Result<(BarrelChain)> {
        Ok(BarrelChain::from_bytes(self.chain.pull()?))
    }

    pub fn empty_chain(&self) -> std::io::Result<()> {
        self.chain.clean()
    }
}
