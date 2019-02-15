// spaceboy
use crate::thruster::{Guardian, Revolver, Cowboy, Miner, Telescope};

pub struct Spaceboy {
    master: Cowboy,
    revolver: Revolver
}

impl Spaceboy {
    pub fn hello() -> Spaceboy {
        let revolver = Revolver::locate();
        let cowboy = revolver.master().unwrap();

        Spaceboy {
            master: cowboy,
            revolver: revolver
        }
    }

    pub fn balance(&self) {
        println!("{:#?}", self.master.utxo(
            self.master.public.to_bytes(),
            self.revolver.scan_chain().unwrap()
        ));
    }
    
    pub fn scan_chain(&self) {
        println!("{:#?}", self.revolver.scan_chain().unwrap());
    }

    pub fn scan_pool(&self) {
        println!("{:?}", self.revolver.scan_pool());
    }

    pub fn shoot(&self, value: i32, to: [u8; 32], msg: &'static str) -> std::io::Result<()> {
        let tx = &self.master.load(value, to, msg);

        self.revolver.push_to_pool(tx.to_bytes())
    }

    /// # Pool
    /// + broadcast fail feedback/option
    /// + pack completed, then, boardcast
    /// + pool is empty
    /// + Transaction verify
    pub fn pack(&self, msg: &'static str) -> Result<(), &'static str> {
        let pool = self.revolver.scan_pool();
        let chain = self.revolver.scan_chain();

        if chain.is_err() {
            &self.revolver.stretch_chain(self.master.genesis());
        }

        match &pool.is_ok() {
            true => {
                let bc = chain.unwrap();
                let pre_hash = bc.0[(&bc.len() - 1 as usize)].head.hash;
                let barrel = self.master.mine(
                    String::from(msg).as_bytes(), pool.unwrap(), pre_hash
                );

                &self.revolver.stretch_chain(barrel.to_bytes());
                assert!(true, self.revolver.empty_pool().is_ok());

                Ok(())
            },
            false => {
                Err("oops, pool doesn't exist, try again!")
            }
        }
    }
}
