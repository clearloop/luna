extern crate rand;
extern crate sha2;
extern crate ed25519_dalek;

use std::fs::File;
use std::ops::Deref;
use std::io::{Write, Read};

use sha2::Sha512;
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;

#[derive(Debug, Default)]
pub struct Account(Keypair);
/// ## Sign & Verify
/// ```rust
/// // sign
/// let a = Account::new();
/// let sign = a.sign(b"I'm message");
///
/// // verify
/// let _, err = a.verify(b"I'm message", &sign);
/// // other code...
/// ```
impl Account {
    pub fn new() -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);
        Account(keypair)
    }

    pub fn address(&self) -> String {
        let bytes = self.public.to_bytes();
        let mut hex = String::new();
        hex.extend(bytes.iter().map(|byte| format!("{:02x}", byte)));
        hex
    }

    pub fn save(&self, path: String) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.to_bytes())
    }

    pub fn load(path: String) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = vec![];

        file.read_to_end(&mut contents)?;

        Ok(Account(Keypair::from_bytes(&contents).unwrap()))
    }
}

impl Deref for Account {
    type Target = Keypair;
    fn deref(&self) -> &Self::Target { &self.0 }
}

fn main() {
    let a = Account::default();
    println!("{:?}", a);
}
