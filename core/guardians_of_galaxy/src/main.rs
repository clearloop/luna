extern crate primitive;
extern crate guardians_of_galaxy;

use guardians_of_galaxy::cipher::Cowboy;

fn main() {
    let cowboy = Cowboy::new();
    let tx = cowboy.send(1, [0_u8; 32], "hello, world");
    println!("{:?}", tx);
}
