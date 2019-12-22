use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Default)]
pub struct Database(pub Mutex<HashMap<String, String>>);

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::{Arc, mpsc};
    
    #[test]
    fn test_db() {
        let (tx, rx) = mpsc::channel();
        let db = Arc::new(Database(Mutex::new(HashMap::new())));

        for i in 0..10 {
            let db = db.clone();
            let tx = tx.clone();
            
            thread::spawn(move|| {
                let mut db = db.0.lock().unwrap();
                tx.send(i).unwrap();
                db.insert(i.to_string(), i.to_string())
            });
        }

        for _ in 0..10 {
            let db = db.clone();
            let db = db.0.lock().unwrap();
            let j = rx.recv().unwrap();
            assert_eq!(&j.to_string(), db.get(&j.to_string()).unwrap());
        }
    }
}
