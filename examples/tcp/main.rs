use std::thread;
use spacejam::prototype::tcp::TcpServer;
    
fn main() {
    let server = TcpServer::new("127.0.0.1:7878");
    let client = TcpServer::new("127.0.0.1:7979");

    let t1 = thread::spawn(|| server.serve().unwrap());
    let t2 = thread::spawn(|| client.send("127.0.0.1:7878", "hello").unwrap());

    t1.join().expect("thread one panicked");
    t2.join().expect("thread two panicked");
}
