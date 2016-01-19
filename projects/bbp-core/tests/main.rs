use BBP::{bbp16, bbp256, PiViewerBase16};

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() {
    println!("BBP[4]: 1th decimal: {:X}", bbp16(1));
    println!("BBP[4]: 10th decimal: {:X}", bbp16(10));
    println!("BBP[4]: 100th decimal: {:X}", bbp16(100));
    println!("BBP[4]: 1000th decimal: {:X}", bbp16(1000));
    println!("BBP[4]: 8196th decimal: {:X}", bbp16(8196));
    // 32bit Overflow
    println!("BBP[4]: 10000th decimal: {:X}", bbp16(10000));
    println!("BBP[4]: 100000th decimal: {:X}", bbp16(100000));
    println!("BBP[4]: 1000000th decimal: {:X}", bbp16(1000000));
    // println!("BBP[4]: 10000000th decimal: {:X}", bbp16(10000000));
}

#[test]
fn print16() {
    println!("{}", PiViewerBase16::new(0, 120));
}