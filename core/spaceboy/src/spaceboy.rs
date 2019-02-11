// spaceboy
use thruster::{Guardian, Revolver, Cowboy, Miner};

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

    pub fn shoot(&self) -> std::io::Result<()> {
        let tx = &self.master.load(1, [0_u8;32], "hello");

        self.revolver.pool.push(&tx.to_bytes())
    }

    pub fn pack(&self) -> std::io::Result<()> {
        let pool = self.revolver.pool.pull();
        println!("pool: {:?}", &pool.unwrap());
        
        let res = self.master.mine(
            b"hello, world", vec![], 0,
            self.master.public.to_bytes()
        );
        
        println!("{:?}", &res);
        Ok(())
    }
}
