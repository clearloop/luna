use super::tcp::TcpServer;
use super::handler::handler;

pub fn launch(addr: &'static str) {
    let sj = TcpServer::new(addr);
    sj.serve(handler).unwrap();
}
