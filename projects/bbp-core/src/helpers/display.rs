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
            write!(f, "│ ")?;

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
            write!(f, "│ ")?;

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
            let position = self.start as usize + i * 10;
            write!(f, "{}", position)?;
            for _ in 0..(left_length - position.length()) {
                write!(f, " ")?;
            }
            write!(f, "│ ")?;

            for base10 in chunk.iter() {
                for _ in 0..(max_width - base10.length()) {
                    write!(f, " ")?;
                }
                write!(f, "{}", base10)?;
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
