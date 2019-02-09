// db
use std::fs::{File, create_dir, OpenOptions};
use std::io::{Write, Read};
use std::path::PathBuf;

/// # IO
/// need data structure that stand alone
/// real push and pull
pub struct IO {
    path: PathBuf
}

impl IO {
    pub fn create_dir(path: &PathBuf) -> std::io::Result<()> {
        create_dir(path)?;
        Ok(())
    }
    
    pub fn locate(path: &PathBuf) -> Self {
        IO { path: path.to_path_buf() }
    }

    pub fn exists(&self) -> bool {
        self.path.as_path().exists()
    }    
    
    pub fn push<B>(&self, data: B) -> std::io::Result<()>
    where B: std::convert::AsRef<[u8]>
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.path)?;

        file.write_all(data.as_ref())?;
        file.flush()?;
        Ok(())
    }

    pub fn pull(&self) -> std::io::Result<(Vec<u8>)> {
        let mut file = File::open(&self.path)?;
        let mut content = vec![];

        file.read_to_end(&mut content).unwrap();
        Ok(content)
    }
}
