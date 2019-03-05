// test
use core::primitive::Capsule;

fn main() {
    let mut capsule = Capsule::bind("127.0.0.1:1439", vec![]);
    capsule.update(vec![
        ("127.0.0.1:1440", false),
        ("127.0.0.1:3055", false),
        ("127.0.0.1:3326", false)
    ]);
    
    capsule.gossip();
}
