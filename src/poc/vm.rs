use std::io::Result;
use ketos::{Interpreter, Value, FromValueRef};
use super::db::Database;

/// Vm in SpaceJam
///
/// Actually a lisp interpreter here it is.
///
/// TODO:
/// Convert code into syn, offer multi-language api.
#[derive(Default)]
pub struct Vm(pub Database);

macro_rules! ketos_ref_ok {
    ($tt:ident, $res:ident) => {
        Ok($tt::from_value_ref(&$res).unwrap().to_string())
    }
}

impl Vm {
    pub fn input(&self, name: &'static str, code: &'static str) -> Result<()> {
        let mut db = (self.0).0.lock().unwrap();
        db.insert(name.to_string(), code.to_string());
        Ok(())
    }

    pub fn exec(&self, name: &'static str, params: Vec<Value>) -> Result<String> {
        let interp = Interpreter::new();
        let db = (&self.0).0.lock().unwrap();
        let code = db.get(name).unwrap();

        interp.run_code(code, None).unwrap();
        let res = interp.call(name, params).unwrap();
        
        match res.type_name() {
            "uint" => ketos_ref_ok!(usize, res),
            "bool" => ketos_ref_ok!(bool, res),
            "float" => ketos_ref_ok!(f32, res),
            "integer" => ketos_ref_ok!(i32, res),
            "char" => ketos_ref_ok!(char, res),
            "string" => {
                match res {
                    Value::String(s) => Ok(s.into_string()),
                    _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "exec error"))
                }
            },
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "exec error"))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let vm = Vm::default();
        let code = r#"(define (twoSum x y) (+ x y))"#;

        &vm.input("twoSum", code);
        let res = vm.exec("twoSum", vec![99.into(), 1.into()]);
        assert_eq!(100.to_string(), res.unwrap());
    }
}
