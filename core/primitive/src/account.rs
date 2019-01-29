use std::ops::{Deref, DerefMut};
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;

/// Users Flow Chart
/// Load Account
/// -> Make Transaction
/// -> Send Transaction to Transaction Pool
/// -> Boarcast
/// -> Transaction Pool Call Back?
/// -> <Minner>
pub struct Account(Keypair);
impl Account {
    pub fn new() -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Account(keypair)
    }

    pub fn send() -> std::io::Result<()> {
        Ok(())
    }
}

impl Deref for Account {
    type Target = Keypair;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Account {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
