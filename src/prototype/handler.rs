use std::net::TcpStream;
use std::io::{Read, Write};
use std::cell::RefCell;
use std::sync::Arc;
use super::tcp::TCP_PACKAGE_SIZE;
use super::vm::Vm;
use ketos::Value;

thread_local!(static VM: RefCell<Vm> = RefCell::new(Vm::default()));

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
///   let c = Request::from("C(twoSum) (define (twoSum x y) (+ x y))");
///   let q = Request::from("Q(twoSum) (8 32)");
///   assert_eq!(c, Request{ prefix: Prefix::Contract, func: "twoSum", expr: "(define (twoSum x y) (+ x y))"});
///   assert_eq!(q, Request{ prefix: Prefix::Query, func: "twoSum", expr: "(8 32)"});
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

        if (ptr == 0) | (es == 0) {
            return Request {
                expr: "",
                func: "",
                prefix: Prefix::Error
            };
        }
        
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

/// Handle tcp requests
///
/// Just can parse reqs with int params.
///
/// TODO:
/// Add multi type parser.
pub fn handler(mut stream: TcpStream, vm: Arc<Vm>) {
    let mut recv = [0; TCP_PACKAGE_SIZE];
    stream.read(&mut recv).unwrap();

    let text: &'static str = Box::leak(
        std::str::from_utf8(&recv.to_vec())
            .unwrap()
            .trim_matches(char::from(0))
            .to_string()
            .into_boxed_str()
    );

    let req = Request::from(text);
    match req.prefix {
        Prefix::Contract => {
            if vm.input(req.func, req.expr).is_ok() {
                stream.write(b"ok").unwrap();
            }
        },
        Prefix::Query => {
            let expr: Vec<i32> = req.expr.split(' ').map(
                |x| x.parse::<i32>().unwrap_or(0)
            ).collect();
            let params: Vec<Value> = expr.iter().map(|x| Value::from(*x)).collect();

            let res = vm.exec(req.func, params);
            if res.is_ok() {
                stream.write(b"ok").unwrap();
            }
        },
        Prefix::Error => {
            stream.write(b"err").unwrap();
        }
    }
    
    // stream.write(format!("{:?}", text.to_owned()).as_bytes()).unwrap();
    // println!("[Server] received: {:?}", text);
    // stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::{Prefix, Request};
    
    #[test]
    fn test_req_parser() {
        let c = Request::from("C(twoSum) (define (twoSum x y) (+ x y))");
        let q = Request::from("Q(twoSum) (8 32)");
        assert_eq!(c, Request{ prefix: Prefix::Contract, func: "twoSum", expr: "(define (twoSum x y) (+ x y))"});
        assert_eq!(q, Request{ prefix: Prefix::Query, func: "twoSum", expr: "(8 32)"});
    }
}
