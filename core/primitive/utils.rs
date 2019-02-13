// utils
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

///# Utils
///+ H512
///+ Timestamp
///+ Disk I/O
///+ Convert
//@H512 
pub fn hmac<B>(data: B) -> [u8;32]
where B: std::convert::AsRef<[u8]>
{
    let mut hasher = Sha256::new();
    let mut hmac = <[u8;32]>::default();

    hasher.input(data.as_ref());
    hmac.copy_from_slice(&hasher.result()[..]);

    hmac
}

//@Timestamp
pub fn ts() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Take your protein pills and put your helmet on.");

    return now.as_secs() * 1000 + now.subsec_millis() as u64;
}

//@Convert
pub fn hex<B>(bytes: B) -> String
where B: std::convert::AsRef<[u8]> {
    let mut hex = String::new();
    hex.extend(bytes.as_ref().iter().map(|byte| format!("{:02x}", byte)));
    hex
}
