use std::thread;
use spacejam::prototype::tcp::TcpServer;
use spacejam::prototype::handler::handler;
    
fn main() {
    let server = TcpServer::new("127.0.0.1:7878");
    let client = TcpServer::new("127.0.0.1:7979");
    let client2 = TcpServer::new("127.0.0.1:7980");

    let t1 = thread::spawn(|| server.serve(handler).unwrap());
    let t2 = thread::spawn(|| client.send(
        "127.0.0.1:7878",
        r#"C(twoSum) (define (twoSum x y) (+ x y))"#
    ).unwrap());

    let t3 = thread::spawn(|| client2.send(
        "127.0.0.1:7878",
        r#"Q(twoSum) (2 2)"#
    ).unwrap());
    
    let r1 = t1.join().expect("thread one panicked");
    let r2 = t2.join().expect("thread two panicked");
    let r3 = t3.join().expect("thread two panicked");
    
    println!("{:?}", r1);
    println!("{:?}", r2);
    println!("{:?}", r3);
}
