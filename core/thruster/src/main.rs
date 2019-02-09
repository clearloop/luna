extern crate thruster;
use thruster::revolver::Revolver;

fn main() {
    let mgz = Revolver::locate();
    let exists = mgz.cowboy.exists();
    match exists {
        true => println!("hi"),
        false => println!("hello")
    }
}
