use crate::{bytes, deref};
use std::ops::{Deref, DerefMut};
use rand::rngs::OsRng;
use ed25519_dalek::{Keypair};
use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize};

/// Cowboy has public key and secret key.
#[derive(Serialize, Deserialize, Debug)]
pub struct Cowboy(Keypair);
impl Cowboy {
    pub fn born() -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Cowboy(keypair)
    }
}

bytes!(Cowboy);
deref!(Cowboy, Keypair);

#[cfg(test)]
mod tests {
    use super::Cowboy;
    #[test]
    fn unique_cowboy() {
        assert_ne!(Cowboy::born().to_bytes(), Cowboy::born().to_bytes());
    }
}
