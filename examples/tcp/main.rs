use std::thread;
use spacejam::prototype::tcp::TcpServer;
use spacejam::prototype::handler::handler;
    
fn main() {
    let server = TcpServer::new("127.0.0.1:7878");
    let client = TcpServer::new("127.0.0.1:7979");

    let t1 = thread::spawn(|| server.serve(handler).unwrap());
    let t2 = thread::spawn(|| client.send("127.0.0.1:7878", "hello").unwrap());
    
    let r1 = t1.join().expect("thread one panicked");
    let r2 = t2.join().expect("thread two panicked");
    
    println!("{:?}", r1);
    println!("{:?}", r2);
}
