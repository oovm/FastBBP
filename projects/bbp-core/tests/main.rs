use BBP::{bbp16, PiViewerBase16, PiViewerBase256};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    assert_eq!(bbp16(0), 0x2);
    assert_eq!(bbp16(10), 0xA);
    assert_eq!(bbp16(100), 0x2);
    assert_eq!(bbp16(1000), 0x4);
    assert_eq!(bbp16(8196), 0xA);
}

#[test]
fn print16() {
    println!("{}", PiViewerBase16::new(0, 120));
    println!("{:x}", PiViewerBase16::new(0, 120));
    println!("{:X}", PiViewerBase16::new(0, 120));

    println!("{}", PiViewerBase256::new(0, 120));
    println!("{:x}", PiViewerBase256::new(0, 120));
    println!("{:X}", PiViewerBase256::new(0, 120));
}

#[test]
fn print32() {
    println!("{}", PiViewerBase16::new(10000, 120));
    println!("{:x}", PiViewerBase16::new(10000, 120));
    println!("{:X}", PiViewerBase16::new(10000, 120));
}
