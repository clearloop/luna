// io
use std::path::PathBuf;
use std::io::{Write, Read};
use std::fs::{File, create_dir, OpenOptions};

/// # IO
/// need data structure that stand alone
/// real push and pull
#[derive(Clone)]
pub struct IO {
    path: PathBuf
}

impl IO {
    //@test: create dir, create file
    /// ErrKind: Write Authority, Bubble
    pub fn locate(path: &'static str) -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let luna = home_dir.join(".luna");
        if luna.exists() == false {
            create_dir(&luna).unwrap();
        }
        
        let target = luna.join(path);
        File::create(&target).unwrap();
        
        IO { path: target }
    }

    //@test: remove file
    /// ErrKind: Write Authority, `unwrap()` directly.
    pub fn clean(&self) {
        std::fs::remove_file(&self.path).unwrap()
    }

    //@test: write file
    /// ErrKind: Write Authority, __Bubble__
    pub fn push<B>(&self, data: B) 
    where B: std::convert::AsRef<[u8]> {
        let mut file = OpenOptions::new()
            .write(true).open(&self.path).unwrap();

        file.write_all(data.as_ref()).unwrap();
        file.flush().unwrap()
    }

    //@test: read file
    /// ErrKind: Read Authority, Redirect to create and write.
    pub fn pull(&self) -> Vec<u8> {
        let mut file = File::open(&self.path).unwrap();
        let mut content = vec![];

        file.read_to_end(&mut content).unwrap();
        content
    }
}

#[cfg(test)]
mod tests {
    use super::IO;
    #[test]
    fn handler() {
        let file = IO::locate("test_file");
        assert_eq!(file.path.exists(), true);

        file.push(b"halo, spaceboy");
        assert_eq!(file.pull(), b"halo, spaceboy");

        file.clean();
        assert_eq!(file.path.exists(), false);
    }
}
