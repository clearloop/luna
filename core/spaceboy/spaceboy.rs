// spaceboy
use crate::thruster::{Guardian, Revolver, Cowboy, Miner};

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

    pub fn shoot(&self, value: i32, to: [u8; 32], msg: &'static str) -> std::io::Result<()> {
        let tx = &self.master.load(value, to, msg);
        
        self.revolver.push_to_pool(tx.to_bytes())
    }

    /// # Pool
    /// + broadcast fail feedback/option
    /// + pack completed, then, boardcast
    /// + pool is empty
    pub fn pack(&self) -> std::io::Result<(usize, Vec<u8>)> {
        let pool = self.revolver.scan_pool();
        let res: std::io::Result<(usize, Vec<u8>)>;

        // if pool exists
        match &pool.is_ok() {
            true => {
                let pool_bytes = pool.unwrap();
                res = Ok(self.master.mine(
                    b"hello, world", pool_bytes, 0,
                    self.master.public.to_bytes()
                ))
            },
            false => {
                res = Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "oops, pool doesn't exist, try again!")
                )
            }
        }
        
        let _ = self.revolver.empty_pool();
        res
    }
}
