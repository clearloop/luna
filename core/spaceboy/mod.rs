// spaceboy
use crate::thruster::{Guardian, Revolver, Cowboy, Miner, Telescope};

pub struct Spaceboy {
    master: Cowboy,
    revolver: Revolver
}

impl Spaceboy {
    pub fn hello() -> Spaceboy {
        let revolver = Revolver::locate();
        let cowboy = revolver.master();
    
        Spaceboy {
            master: cowboy,
            revolver: revolver
        }
    }

    pub fn balance(&mut self) {
        println!("{:#?}", self.master.utxo(
            self.master.public.to_bytes(),
            self.revolver.chain.to_owned()
        ));
    }

    pub fn transfer(&mut self, value: i32, to: [u8; 32], msg: &'static str) {
        let tx = self.master.load(value, to, msg);
        self.revolver.shoot(tx);
    }
    
    pub fn pack(&mut self, msg: &'static str) {
        let bc = &self.revolver.chain;
        let pre_hash = bc.0[(&bc.len() - 1 as usize)].head.hash;

        let barrel = &self.master.mine(
            String::from(msg).as_bytes(), self.revolver.pool.to_owned(), pre_hash
        );

        &self.revolver.revolve(barrel.to_owned());
    }
}
