use std::net::TcpStream;
use std::io::{Read, Write};
use super::tcp::TCP_PACKAGE_SIZE;

#[derive(Debug, PartialEq)]
pub enum Prefix {
    Contract,
    Query,
    Error
}

/// Format Tcp requests
///
/// for example:
/// ```
/// use spacejam::prototype::handler::{Request, Prefix};
/// 
/// fn main() {
///     let req = "C(twoSum) (define (twoSum x y) (+ x y))";
///     let res = Request::from(req);
///     assert_eq!(res, Request{ prefix: Prefix::Contract, func: "twoSum", expr: "(define (twoSum x y) (+ x y))"});
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct Request {
    pub prefix: Prefix,
    pub func: &'static str,
    pub expr: &'static str
}

impl std::convert::From<&'static str> for Request {
    fn from(s: &'static str) -> Self {
        let ptr = s.find(")").unwrap_or(0);
        let es = s[ptr..s.len()].find("(").unwrap_or(0) + ptr;
        
        Request {
            prefix: match s.chars().nth(0).unwrap() {
                'C' => Prefix::Contract,
                'Q' => Prefix::Query,
                _ => Prefix::Error
            },
            func: &s[2..ptr],
            expr: &s[es..s.len()],
        }
    }
}

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


#[cfg(test)]
mod tests {
    use super::{Prefix, Request};
    
    #[test]
    fn test_req_parser() {
        let req = "C(twoSum) (define (twoSum x y) (+ x y))";
        let res = Request::from(req);
        assert_eq!(res, Request{ prefix: Prefix::Contract, func: "twoSum", expr: "(define (twoSum x y) (+ x y))"});
    }
}
