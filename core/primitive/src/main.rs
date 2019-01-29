// test
use primitive::chain::Block;
use primitive::pow::ProofOfWork;
use bincode::serialize;
use primitive::utils::{hmac, hex};

fn main() {
    let genesis = Block::genesis();
    let mut pow = ProofOfWork::new(serialize(&genesis).unwrap(), 10);
    pow.run();
    // println!("0x{}", hex(hmac(pow.prepare_data(10))));
}
