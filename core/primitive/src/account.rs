use crate::deref;
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
#[derive(Debug)]
pub struct Account(Keypair);
impl Account {
    pub fn new() -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Account(keypair)
    }
}

deref!(Account, Keypair);
