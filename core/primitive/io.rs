// db
use std::path::PathBuf;
use std::io::{Write, Read};
use std::fs::{File, create_dir, OpenOptions};

/// # IO
/// need data structure that stand alone
/// real push and pull
pub struct IO {
    path: PathBuf
}

impl IO {
    pub fn locate(path: &PathBuf) -> Self {
        IO { path: path.to_path_buf() }
    }

    pub fn clean(&self) -> std::io::Result<()> {
        match &self.path.as_path().is_dir() {
            true => std::fs::remove_dir(&self.path),
            false => std::fs::remove_file(&self.path),
        }
        
    }

    pub fn exists(&self) -> bool {
        self.path.as_path().exists()
    }
    
    pub fn home_dir() -> PathBuf {
        dirs::home_dir().unwrap()
    }

    pub fn create_dir(path: &PathBuf) -> std::io::Result<()> {
        create_dir(path)?;
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::IO;
    #[test]
    fn handler() {
        let home = IO::home_dir();
        let luna = home.join(".luna");
        
        let dir = IO::locate(&luna.join("tests"));
        assert_eq!(dir.exists(), false);
        assert_eq!(IO::create_dir(&dir.path).is_ok(), true);
        assert_eq!(dir.exists(), true);
        assert_eq!(dir.clean().is_ok(), true);
        assert_eq!(dir.exists(), false);
        
        let file = IO::locate(&luna.join("tests"));
        assert_eq!(file.exists(), false);
        assert_eq!(file.push(b"halo, spaceboy").is_ok(), true);
        assert_eq!(file.exists(), true);
        assert_eq!(file.pull().unwrap(), b"halo, spaceboy");
        assert_eq!(file.clean().is_ok(), true);
        assert_eq!(file.exists(), false);
    }    
}
