// db
use std::fs::File;
use std::io::{Write, Read};

pub struct DB {
    path: &'static str
}

impl DB {
    pub fn new(path: &'static str) -> Self {
        DB { path: path }
    }
    
    pub fn write<B>(&self, data: B) -> std::io::Result<()>
    where B: std::convert::AsRef<[u8]>
    {
        let mut file = File::create(self.path)?;
        file.write_all(data.as_ref())
    }

    pub fn read(&self) -> Vec<u8> {
        let mut file = File::open(self.path).unwrap();
        let mut content = vec![];

        file.read_to_end(&mut content).unwrap();
        content
    }
}
