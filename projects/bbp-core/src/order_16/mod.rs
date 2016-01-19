use std::fmt::{Display, Formatter, Write};
use std::sync::mpsc;
use std::thread;
use crate::helpers::pow_mod;

#[derive(Clone, Debug, Default)]
pub struct PiViewerBase16 {
    start: u64,
    buffer: Vec<u8>,
}

impl PiViewerBase16 {
    pub fn new(start: u64, length: u64) -> Self {
        let mut buffer = vec![0; length as usize];
        for delta in 0..length {
            let index = delta + start;
            let digit = bbp16(index);
            unsafe {
                *buffer.get_unchecked_mut(delta as usize) = digit;
            }
        }
        Self { start, buffer }
    }
}

impl Display for PiViewerBase16 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let max_length = (self.start + self.buffer.len() as u64).to_string().len();

        for (i, chunk) in self.buffer.chunks(16).enumerate() {
            let position = self.start as usize + i * 16;
            write!(f, "{}", position)?;
            for _ in 0..(max_length - position.to_string().len()) {
                write!(f, " ")?;
            }
            write!(f, "| ")?;

            for (j, base256) in chunk.iter().enumerate() {
                write!(f, "{:02X}", base256)?;
                match j % 4 {
                    3 => write!(f, "  ")?,
                    _ => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


pub fn bbp16(digit: u64) -> u8 {
    let mut f = [(1, 4.0), (4, -2.0), (5, -1.0), (6, -1.0)].iter().map(|&(j, k)| k * series_sum(digit, j)).sum::<f64>();
    ((f - f.floor()) * 16.0).floor() as u8
}

fn series_sum(digit: u64, j: u64) -> f64 {
    let fraction1: f64 = (0..digit + 1)
        .map(|i| pow_mod(16, digit - i, 8 * i + j) as f64 / (8 * i + j) as f64)
        .fold(0.0, |x, y| (x + y).fract());
    let fraction2: f64 = (digit + 1..)
        .map(|i| 16.0_f64.powi(-((i - digit) as i32)) / ((8 * i + j) as f64))
        .take_while(|&x| x.abs() > 1e-13_f64)
        .sum();
    fraction1 + fraction2
}
