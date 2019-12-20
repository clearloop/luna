use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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
        
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let mut recv = vec![];

            stream.read_to_end(&mut recv)?;
            stream.write(format!("recivecd {:?}", recv).as_bytes())?;
            stream.flush().unwrap();
            
            println!(
                "[Server] received: {:?}",
                String::from_utf8(recv.to_vec()).unwrap()
            );
        }

        Ok(())
    }

    pub fn send(self, addr: &'static str, content: &'static str) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write(content.as_bytes())?;
        Ok(())
    }
}
