// test
// use core::spaceboy::Spaceboy;
use core::primitive::Barrel;

fn main() {
    // let sb = Spaceboy::hello();
    // sb.balance();
    // let _ = sb.shoot(1, [0_u8;32], "hello, spaceboy");
    // let res = sb.pack("Crucify Your Mind");
    // println!("{:?}", &res.is_ok());
    println!("{:#?}", Barrel::new(b"halo, spaceboy", vec![], [0_u8; 32], [0_u8; 32]));
}
