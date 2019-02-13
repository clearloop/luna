use core::spaceboy::Spaceboy;

fn main() {
    let sb = Spaceboy::hello();
    let _ = sb.shoot(1, [0_u8;32], "hello, aaa");
    let res = sb.pack("Crucify Your Mind");
    println!("{:?}", &res.unwrap());
}
