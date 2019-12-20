// spaceboy
use crate::thruster::{Guardian, Revolver, Cowboy, Miner, Telescope};

/// Hallo Spaceboy
pub struct Spaceboy {
    master: Cowboy,
    pub revolver: Revolver
}

impl Spaceboy {
    /// # Example
    /// ```
    /// use spacejam::Spaceboy;
    ///
    /// fn main() {
    ///     let mut spaceboy = Spaceboy::new(
    ///         "test_pool_spaceboy",
    ///         "test_chain_spaceboy",
    ///         "test_cowboy_spaceboy"
    ///     );
    ///     spaceboy.transfer(10, [0_u8;32], "halo, spaceboy");
    ///     spaceboy.pack("pack");
    ///     spaceboy.revolver.explode();
    /// }
    /// ```
    pub fn new(pool: &'static str, chain: &'static str, cowboy: &'static str) -> Spaceboy {
        let revolver = Revolver::locate(pool, chain, cowboy);
        let cowboy = revolver.master();
    
        Spaceboy {
            master: cowboy,
            revolver: revolver
        }
    }

    /// generate database
    pub fn hello() -> Spaceboy {
        Spaceboy::new("pool", "chain", "cowboy")
    }

    /// checkout utxo
    pub fn watch(&mut self, account: [u8;32]) -> usize {
        self.master.utxo(
            account, self.revolver.chain.to_owned()
        )
    }

    /// checkout balance
    pub fn balance(&mut self) -> usize {
        self.master.utxo(
            self.master.public.to_bytes(),
            self.revolver.chain.to_owned()
        )
    }    

    /// transfer tx to others.
    pub fn transfer(&mut self, value: i32, to: [u8; 32], msg: &'static str) {
        let tx = self.master.load(value, to, msg);
        self.revolver.shoot(tx);
    }

    /// Pack barrel to send
    pub fn pack(&mut self, msg: &'static str) {
        let bc = &self.revolver.chain;
        let pre_hash = bc[(bc.len() -1) as usize].head.hash;

        let barrel = &self.master.mine(
            String::from(msg).as_bytes(), self.revolver.pool.to_owned(), pre_hash
        );

        &self.revolver.revolve(barrel.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transfer_and_mine() {
        let mut spaceboy = Spaceboy::new(
            "test_pool_spaceboy",
            "test_chain_spaceboy",
            "test_cowboy_spaceboy"
        );
        spaceboy.transfer(10, [0_u8;32], "halo, spaceboy");
        spaceboy.pack("pack");
        spaceboy.revolver.explode();
    }
}
