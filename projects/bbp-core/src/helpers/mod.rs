use core::{
    fmt::{Display, Formatter},
    ops::{Div, Mul, Rem},
};
use std::ops::Add;
mod display;

pub(crate) trait DigitLength {
    fn length(&self) -> usize;
}
impl DigitLength for u8 {
    fn length(&self) -> usize {
        self.checked_ilog10().unwrap_or(0).add(1) as usize
    }
}
impl DigitLength for u64 {
    fn length(&self) -> usize {
        self.checked_ilog10().unwrap_or(0).add(1) as usize
    }
}

impl DigitLength for usize {
    fn length(&self) -> usize {
        self.checked_ilog10().unwrap_or(0).add(1) as usize
    }
}

pub fn pow_mod(n: u64, m: u64, d: u64) -> u64 {
    // n * k^2 < 2^64 - 1
    if n < 100 && d < 400_000_000 { pow_mod_generic(n, m, d) } else { pow_mod_generic(n as u128, m as u128, d as u128) as u64 }
}

fn pow_mod_generic<T>(n: T, m: T, d: T) -> T
where
    T: Copy + PartialEq + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + From<u64>,
{
    if m == 0.into() {
        if d == 1.into() { 0.into() } else { 1.into() }
    }
    else if m == 1.into() {
        n % d
    }
    else {
        let k = pow_mod_generic(n, m / 2.into(), d);
        if m % 2.into() == 0.into() { (k * k) % d } else { (k * k * n) % d }
    }
}

pub(crate) struct DecViewer<'i, T> {
    pub start: u64,
    pub buffer: &'i [T],
}

pub(crate) struct HexViewer8<'i> {
    pub lower: bool,
    pub start: u64,
    pub buffer: &'i [u8],
}

pub(crate) struct HexViewer16<'i> {
    pub lower: bool,
    pub start: u64,
    pub buffer: &'i [u16],
}
