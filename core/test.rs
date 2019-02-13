use core::spaceboy::Spaceboy;

fn main() {
    let sb = Spaceboy::hello();
    // sb.shoot(1, [0_u8;32], "hello, spaceboy");
    let _ = sb.shoot(1, [0_u8;32], "hello, aaa");
    let res = sb.pack();
    println!("{:?}", &res);
}
