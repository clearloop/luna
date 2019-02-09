// Revolver
pub use primitive::cowboy::Cowboy;
use primitive::io::IO;

///IO Efficiency
///TODO: files' size, just like, 00, 01...
pub struct Revolver {
    pub pool:  IO,
    pub chain: IO,
    pub state: IO,
    pub cowboy: IO
}

impl Revolver {
    pub fn locate() -> Self {
        let home = dirs::home_dir().unwrap();
        let _ = IO::create_dir(&home.join(".luna"));
        let luna = &home.join(".luna");        

        Revolver {
            pool:  IO::locate(&luna.join("pool")),
            chain: IO::locate(&luna.join("chain")),
            state: IO::locate(&luna.join("state")),
            cowboy: IO::locate(&luna.join("cowboy")),
        }
    }

    pub fn master(&self) -> std::io::Result<Cowboy> {
        if &self.cowboy.exists() == &false {
            &self.cowboy.push(Cowboy::born().to_bytes())?;
        }
        
        Ok(Cowboy::from_bytes(self.cowboy.pull()?))
    }
}
