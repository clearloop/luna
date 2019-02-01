extern crate primitive;
extern crate spaceship;

use spaceship::cipher::Transfer;

fn main() {
    let transfer = Transfer::new();
    let tx = transfer.generate(1, [0_u8; 32], "hello, world");
    println!("{:?}", tx);
    transfer.verify("hello, world", tx);
}
