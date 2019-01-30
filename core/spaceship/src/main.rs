extern crate primitive;
extern crate spaceship;
// use primitive::{ account::Account, utils::hex};
use spaceship::txs::{Transaction, Symmetric};

fn main() {
    // let account = Account::new();
    let tx = Transaction::new(vec![], vec![], vec![]);
    println!("Transaction: {:?}", tx);
}
