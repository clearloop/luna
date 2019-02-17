// Revolver
use super::Cowboy;
use crate::primitive::{IO, Transaction, TransactionArray, Barrel, BarrelChain};

#[derive(Clone)]
struct Wheel {
    pub pool:  IO,
    pub chain: IO,
    pub cowboy: IO
}

impl Wheel {
    pub fn new(
        pool: &'static str,
        chain: &'static str,
        cowboy: &'static str
    ) -> Self {
        let pool = IO::locate(pool);
        let chain = IO::locate(chain);
        let cowboy = IO::locate(cowboy);

        Wheel {
            pool: pool,
            chain: chain,
            cowboy: cowboy
        }
    }

    /// ErrKind: can't convert
    pub fn master(&self) -> Cowboy {
        if self.cowboy.pull().len() == 0 {
            self.cowboy.push(Cowboy::born().to_bytes());
        }
        
        Cowboy::from_bytes(self.cowboy.pull())
    }

    /// ErrKind: can't convert
    pub fn scan_pool(&self) -> TransactionArray {
        if self.pool.pull().len() == 0 {
            self.pool.push(TransactionArray::default().to_bytes())
        }
        
        TransactionArray::from_bytes(self.pool.pull())
    }

    pub fn fill_pool(&self, txs: &TransactionArray) {
            self.pool.push(txs.to_bytes())
    }

    /// ErrKind: can't convert
    pub fn scan_chain(&self) -> BarrelChain {
        if self.chain.pull().len() == 0 {
            let mut bc = BarrelChain::default();
            let mut txs = TransactionArray::default();
            
            let tx = Transaction::reward("Genesis Transaction", [0_u8; 32]);
            txs.push(tx);

            let barrel = Barrel::new("Genesis Block", txs.to_bytes(), [0_u8;32]);
            bc.push(barrel);
            
            self.chain.push(bc.to_bytes());
        }

        BarrelChain::from_bytes(self.chain.pull())
    }

    pub fn stretch_chain(&self, bc: &BarrelChain) {
        self.chain.push(bc.to_bytes())
    }
}

///IO Efficiency
///TODO: files' size, just like, 00, 01...
pub struct Revolver {
    wheel: Wheel,
    pub pool:  TransactionArray,
    pub chain: BarrelChain
}

impl Revolver {
    pub fn locate(p: &'static str, c: &'static str, cb: &'static str) -> Self {
        let wheel = Wheel::new(p, c, cb);

        Revolver {
            wheel: wheel.clone(),
            pool: wheel.scan_pool(),
            chain: wheel.scan_chain()
        }
    }

    pub fn master(&self) -> Cowboy {
        self.wheel.master()
    }

    pub fn shoot(&mut self, tx: Transaction) {
        &self.pool.push(tx);
        &self.wheel.fill_pool(&self.pool);
        self.pool = self.wheel.scan_pool();
    }

    pub fn revolve(&mut self, bc: Barrel) {
        &self.chain.push(bc);
        &self.wheel.stretch_chain(&self.chain);
        self.chain = self.wheel.scan_chain();

        self.wheel.pool.clean();
    }

    pub fn explode(&self) {
        if self.wheel.pool.path.exists() {
            self.wheel.pool.clean();
        }
        
        self.wheel.chain.clean();
        self.wheel.cowboy.clean();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn revolver() {
        let revolver = Revolver::locate("test_pool", "test_chain", "test_cowboy");
        
        assert_eq!(revolver.pool.len(), 0);
        assert_eq!(revolver.pool.to_bytes().len(), 8);
        assert_eq!(revolver.pool, TransactionArray::default());

        assert_eq!(revolver.chain.len(), 1);
        assert_eq!(revolver.chain.to_bytes().len(), 239);

        assert_eq!(revolver.master().to_bytes().len(), 72);
        revolver.explode();
    }
}
