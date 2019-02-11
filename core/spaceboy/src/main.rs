use spaceboy::Spaceboy;

fn main() {
    let res = Spaceboy::hello();
    println!("{:?}", &res.pack());
}
