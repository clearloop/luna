use std::net::TcpStream;
use std::io::{Read, Write};
use super::tcp::TCP_PACKAGE_SIZE;

pub fn handle_connection(mut stream: TcpStream) {
    let mut recv = [0; TCP_PACKAGE_SIZE];
    stream.read(&mut recv).unwrap();
    
    let text = String::from_utf8(
        recv.to_vec()
    ).unwrap().trim_matches(char::from(0)).to_string();
    
    stream.write(format!("{:?}", text.to_owned()).as_bytes()).unwrap();
    println!("[Server] received: {:?}", text);

    stream.flush().unwrap();
}
