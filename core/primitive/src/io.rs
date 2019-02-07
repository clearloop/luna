// db
use std::fs::{File, create_dir};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};

pub struct IO {
    path: PathBuf
}

impl IO {
    pub fn new(path: &PathBuf) -> Self {
        IO { path: path.to_path_buf() }
    }

    pub fn create_dir(path: &PathBuf) -> std::io::Result<()> {
        create_dir(path)?;
        Ok(())
    }
    
    pub fn push<B>(&self, data: B) -> std::io::Result<()>
    where B: std::convert::AsRef<[u8]>
    {
        let mut file: File;
        match Path::new(&self.path).exists() {
            true => {
                file = File::open(&self.path)?;
            },
            _ => {
                file = File::create(&self.path)?;
            }
        }
        file.write_all(data.as_ref())
    }

    pub fn pull(&self) -> Vec<u8> {
        let mut file = File::open(&self.path).unwrap();
        let mut content = vec![];

        file.read_to_end(&mut content).unwrap();
        content
    }
}
