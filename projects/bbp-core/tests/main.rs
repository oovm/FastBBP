use BBP::{bbp16, PiViewer16, PiViewer4, PiViewer8};

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
    println!("{}", PiViewer4::new(0, 120));
    println!("{:x}", PiViewer4::new(0, 120));
    println!("{:X}", PiViewer4::new(0, 120));
}

#[test]
fn print256() {
    // println!("{}", PiViewer16::new(0, 120));
    println!("{:x}", PiViewer16::new(0, 120));
    println!("{:X}", PiViewer8::new(0, 120));
}

#[test]
fn print32() {
    println!("{}", PiViewer4::new(0, 120));
    println!("{}", PiViewer8::new(0, 120));
}
