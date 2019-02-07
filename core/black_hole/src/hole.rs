use primitive::io::IO;

///IO Efficiency
///TODO: files' size, just like, 00, 01...
pub struct Hole {
    pub pool:  IO,
    pub chain: IO,
    pub state: IO
}

impl Hole {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap();
        let _ = IO::create_dir(&home.join(".luna"));
        let luna = &home.join(".luna");
        
        Hole {
            pool:  IO::new(&luna.join("pool")),
            chain: IO::new(&luna.join("chain")),
            state: IO::new(&luna.join("state"))
        }
    }
}
