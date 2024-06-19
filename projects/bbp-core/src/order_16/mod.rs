use crate::helpers::{pow_mod, HexViewer8};
use alloc::{vec, vec::Vec};
use core::fmt::{Display, Formatter, LowerHex, UpperHex};
use std::{path::PathBuf, sync::mpsc, thread};

mod display;

/// ```
/// # use BBP::PiViewer4;
/// println!("{:x}", PiViewer4::new(0, 120))
/// ```
#[derive(Clone, Debug, Default)]
pub struct PiViewer4 {
    start: u64,
    buffer: Vec<u8>,
}

impl PiViewer4 {
    /// Find the hexadecimal digits of pi starting at `start` and ending at `start + length`.
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

/// The order-16 BBP formula.
pub fn bbp16(digit: u64) -> u8 {
    let f = [(1, 4.0), (4, -2.0), (5, -1.0), (6, -1.0)].iter().map(|&(j, k)| k * series_sum(digit, j)).sum::<f64>();
    ((f - f.floor()) * 16.0).floor() as u8
}

fn series_sum(digit: u64, j: u64) -> f64 {
    let sum_mod: f64 =
        (0..digit + 1).map(|i| pow_mod(16, digit - i, 8 * i + j) as f64 / (8 * i + j) as f64).fold(0.0, |x, y| (x + y).fract());
    let residuals: f64 = (digit + 1..)
        .map(|i| 16.0_f64.powi(-((i - digit) as i32)) / ((8 * i + j) as f64))
        .take_while(|&x| x.abs() > 1e-13_f64)
        .sum();
    sum_mod + residuals
}

fn series_sum_para(digit: u64, j: u64) {
    let (sender, receiver) = mpsc::channel();

    for i in 0..digit + 1 {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            sender_clone.send(i).unwrap();
        });
    }

    let mut sum = 0;
    for _ in 1..=10 {
        sum += receiver.recv().unwrap();
    }
}
