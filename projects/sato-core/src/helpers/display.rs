use super::*;

impl<'i> Display for HexViewer8<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let max_length = (self.start as usize + self.buffer.len()).length();

        for (i, chunk) in self.buffer.chunks(16).enumerate() {
            let position = self.start as usize + i * 16;
            write!(f, "{}", position)?;
            for _ in 0..(max_length - position.length()) {
                write!(f, " ")?;
            }
            f.write_str("│ ")?;

            for (j, base16) in chunk.iter().enumerate() {
                if self.lower {
                    write!(f, "{:02x}", base16)?;
                }
                else {
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

impl<'i> Display for HexViewer16<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let max_length = (self.start as usize + self.buffer.len()).length();

        for (i, chunk) in self.buffer.chunks(16).enumerate() {
            let position = self.start as usize + i * 16;
            write!(f, "{}", position)?;
            for _ in 0..(max_length - position.length()) {
                write!(f, " ")?;
            }
            f.write_str("│ ")?;

            for (j, base16) in chunk.iter().enumerate() {
                if self.lower {
                    write!(f, "{:04x}", base16)?;
                }
                else {
                    write!(f, "{:04X}", base16)?;
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

impl<'i, T> Display for DecViewer<'i, T>
where
    T: Display + Ord + DigitLength,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let left_length = (self.start as usize + self.buffer.len()).length();
        let max_width = self.buffer.iter().max().map(DigitLength::length).unwrap_or(1);
        for (i, chunk) in self.buffer.chunks(10).enumerate() {
            let offset = self.start as usize + i * 10;
            Display::fmt(&offset, f)?;
            for _ in offset.length()..left_length {
                f.write_str(" ")?;
            }
            f.write_str("│")?;
            for base10 in chunk.iter() {
                f.write_str(" ")?;
                for _ in base10.length()..max_width {
                    f.write_str(" ")?;
                }
                Display::fmt(&base10, f)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl DigitLength for u8 {
    fn length(&self) -> usize {
        self.checked_ilog10().unwrap_or(0).add(1) as usize
    }
}
impl DigitLength for u16 {
    fn length(&self) -> usize {
        self.checked_ilog10().unwrap_or(0).add(1) as usize
    }
}
impl DigitLength for u32 {
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
