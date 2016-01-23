use crate::helpers::{pow_mod, HexViewer8};
use alloc::{vec, vec::Vec};
use core::fmt::{Display, Formatter, LowerHex, UpperHex};
use num::{bigint::ToBigInt, BigInt, Integer};

mod display;

/// ```
/// # use BBP::PiViewer4;
/// println!("{:x}", PiViewer4::new(0, 120))
/// ```
#[derive(Clone, Debug, Default)]
pub struct Ramanujan1 {}

fn binary_split(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b == &(a + 1) {
        let pab = -(BigInt::from(6) * a - 5) * (BigInt::from(2) * a - 1) * (BigInt::from(6) * a - 1);
        let qab = BigInt::from(10939058860032000u64) * a.pow(3);
        let rab = &pab * (BigInt::from(545140134) * a + 13591409);
        (pab, qab, rab)
    }
    else {
        let m = (a + b).div_floor(&(2.to_bigint().unwrap()));
        let (pam, qam, ram) = binary_split(a, &m);
        let (pmb, qmb, rmb) = binary_split(&m, b);

        let pab = &pam * &pmb;
        let qab = &qam * &qmb;
        let rab = &qmb * ram + &pam * rmb;
        (pab, qab, rab)
    }
}

fn chudnovsky(n: u32) -> BigInt {
    let (p1n, q1n, r1n) = binary_split(&1.to_bigint().unwrap(), &n.to_bigint().unwrap());
    let multiplier = BigInt::from(426880) * BigInt::from(10005).sqrt().unwrap();
    (multiplier * q1n) / (BigInt::from(13591409) * &q1n + r1n)
}

#[test]
fn main() {
    println!("{:?}", chudnovsky(2)); // this will print an approximation of π
}
