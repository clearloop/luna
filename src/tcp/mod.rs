use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod pool;
use pool::ThreadPool;
/// consts
const TCP_PACKAGE_SIZE: usize = 512;

/// SJ TcpServer
pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub fn new(addr: &'static str) -> TcpServer {
        TcpServer {
            listener: TcpListener::bind(addr).unwrap()
        }
    }

    pub fn serve(self) -> std::io::Result<()> {
        println!("TCP server start at {}", self.listener.local_addr().unwrap());

        let pool = ThreadPool::new(4);
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| {
                handle_connection(stream);
            });
        }

        Ok(())
    }

    pub fn send(self, addr: &'static str, content: &'static str) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write(content.as_bytes())?;

        let mut data = [0; TCP_PACKAGE_SIZE];
        stream.read(&mut data)?;

        let text = String::from_utf8(data.to_vec()).unwrap();
        println!("[Client] received: {}", text);

        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut recv = [0; TCP_PACKAGE_SIZE];
    stream.read(&mut recv).unwrap();
    
    let text = String::from_utf8(
        recv.to_vec()
    ).unwrap().trim_matches(char::from(0)).to_string();
    
    stream.write(format!("{:?}", text.to_owned()).as_bytes()).unwrap();
    println!("[Server] received: {:?}", text);

    stream.flush().unwrap();
}
