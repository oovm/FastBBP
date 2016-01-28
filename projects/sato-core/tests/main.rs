use chudnovsky::{chudnovsky, RamanujanL1};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() {
    println!("{}", RamanujanL1::J163);
    println!("{}", chudnovsky(100));
}
