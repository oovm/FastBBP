use crate::helpers::pow_mod;
use std::fmt::{Display, Formatter, LowerHex, UpperHex};

mod display;

///
#[derive(Clone, Debug, Default)]
pub struct PiViewer8 {
    start: u64,
    buffer: Vec<u8>,
}

impl PiViewer8 {
    /// Find the hexadecimal digits of pi starting at `start` and ending at `start + length`.
    pub fn new(start: u64, length: u64) -> Self {
        let mut buffer = vec![0; length as usize];
        for delta in 0..length {
            let index = delta + start;
            let digit = bbp256(index);
            unsafe {
                *buffer.get_unchecked_mut(delta as usize) = digit;
            }
        }
        Self { start, buffer }
    }
}

/// The order-256 BBP formula.
pub fn bbp256(digit: u64) -> u8 {
    let f = [(1, 256.0), (4, -128.0), (5, -64.0), (6, -64.0), (9, 16.0), (12, -8.0), (13, -4.0), (14, -4.0)]
        .iter()
        .map(|&(j, k)| k * series_sum(digit, j))
        .sum::<f64>();
    ((f - f.floor()) * 256.0).floor() as u8
}

fn series_sum(digit: u64, j: u64) -> f64 {
    let fraction1: f64 = (0..digit + 1)
        .map(|i| pow_mod(256, digit - i, 16 * i + j) as f64 / (16 * i + j) as f64)
        .fold(0.0, |x, y| (x + y).fract());
    let fraction2: f64 = (digit + 1..)
        .map(|i| 256.0_f64.powi(-((i - digit) as i32)) / ((16 * i + j) as f64))
        .take_while(|&x| x.abs() > 1e-16_f64)
        .sum();
    fraction1 + fraction2
}
