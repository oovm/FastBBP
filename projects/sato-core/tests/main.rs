use chudnovsky::{chudnovsky, RamanujanL1};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() {
    println!("{}//N", RamanujanL1::J7);
    println!("{}//N", RamanujanL1::J11);
    println!("{}//N", RamanujanL1::J19);
    println!("{}//N", RamanujanL1::J27);
    println!("{}//N", RamanujanL1::J43);
    println!("{}//N", RamanujanL1::J67);
    println!("{}//N", RamanujanL1::J163);

    println!("{}", chudnovsky(100));
}

#[test]
fn main2() {
    println!("{}", RamanujanL1::J163.run(2));
}
