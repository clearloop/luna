// Capsule
use std::net::{TcpListener, TcpStream};
use rand::{thread_rng, Rng};

/// # Capsule
/// + set up server
/// + Peer List updating
/// + connect once and pass data
/// + pass data and reset lists
/// + time? / if res?
/// + Receive and update peers
/// + Swarm peers
pub struct Capsule {
    pub listener: TcpListener,
    pub peers: Vec<(&'static str, bool)>
}

impl Capsule {
    pub fn bind(address: &'static str, peers: Vec<(&'static str, bool)>) -> Self {
        println!("TCP Serve started at: {}", &address);
        Capsule {
            listener: TcpListener::bind(address).unwrap(),
            peers: peers
        }
    }

    /// how to set thread orders???
    // pub fn handle(&self, func: &Fn(std::net::TcpStream)) {
    //     for stream in self.listener.incoming() {
    //         func(stream.unwrap());
    //     }
    // }
    
    pub fn connect(&self, address: &'static str) {
        if let Ok(stream) = TcpStream::connect(address) {
            println!("Connected to the server!, stream: {:?}", stream);
        } else {
            println!("Couldn't connect to server...");
        }
    }

    pub fn update(&mut self, peers: Vec<(&'static str, bool)>) {
        self.peers = peers;
    }

    /// take threads
    pub fn gossip(&self) {
        let mut _lists: Vec<&'static str> = vec![];
        let mut rng = thread_rng();

        if &self.peers.len() == &0 { return; }
        let peers: Vec<&'static str> = vec![];

        for (idx, _) in peers.iter().enumerate() {
            let n: u8 = rng.gen_range(0, (self.peers.len() - 1) as u8);
            println!("rng: {}", n);

            if idx < (&self.peers.len() / 3) { break; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gossip() {
        let addr = "127.0.0.1:1439";
        let mut capsule = Capsule::bind(addr, vec![]);
        assert_eq!(capsule.listener.local_addr().unwrap().to_string(), addr);

        let peers = vec![
            ("127.0.0.1:1440", false),
            ("127.0.0.1:3055", false),
            ("127.0.0.1:3326", false)];
        capsule.update(peers.clone());
        assert_eq!(peers, capsule.peers);
    }
}
