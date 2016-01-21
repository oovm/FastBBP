use std::fmt::{Display, Formatter};
use std::ops::{Div, Mul, Rem};

pub fn pow_mod(n: u64, m: u64, d: u64) -> u64 {
    // n * k^2 < 2^64 - 1
    if n < 100 && d < 400_000_000 {
        pow_mod_generic(n, m, d)
    } else {
        pow_mod_generic(n as u128, m as u128, d as u128) as u64
    }
}

fn pow_mod_generic<T>(n: T, m: T, d: T) -> T
    where
        T: Copy
        + PartialEq
        + Mul<Output=T>
        + Div<Output=T>
        + Rem<Output=T>
        + From<u64>,
{
    if m == 0.into() {
        if d == 1.into() {
            0.into()
        } else {
            1.into()
        }
    } else if m == 1.into() {
        n % d
    } else {
        let k = pow_mod_generic(n, m / 2.into(), d);
        if m % 2.into() == 0.into() {
            (k * k) % d
        } else {
            (k * k * n) % d
        }
    }
}

pub(crate) struct HexViewer8<'i> {
    pub lower: bool,
    pub start: u64,
    pub buffer: &'i [u8],
}

impl<'i> Display for HexViewer8<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_length = (self.start + self.buffer.len() as u64).to_string().len();

        for (i, chunk) in self.buffer.chunks(16).enumerate() {
            let position = self.start as usize + i * 16;
            write!(f, "{}", position)?;
            for _ in 0..(max_length - position.to_string().len()) {
                write!(f, " ")?;
            }
            write!(f, "│ ")?;

            for (j, base16) in chunk.iter().enumerate() {
                if self.lower {
                    write!(f, "{:02x}", base16)?;
                } else {
                    write!(f, "{:02X}", base16)?;
                }
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