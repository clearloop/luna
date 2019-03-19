// test
use std::thread;
use core::primitive::Capsule;

fn main() {
    let mut _capsule_listener = Capsule::bind("127.0.0.1:1439", vec![]);
    let _capsule_sender = Capsule::bind("127.0.0.1:1440", vec![]);

    thread::spawn(move || {
        _capsule_sender.connect("127.0.0.1:1439");
    });
    
    _capsule_listener.handle( &|stream| {
        println!("{:?}", stream);
    });
}
