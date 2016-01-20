
use super::*;

impl Display for PiViewerBase16 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let max_length = (self.start + self.buffer.len() as u64).to_string().len();

        for (i, chunk) in self.buffer.chunks(10).enumerate() {
            let position = self.start as usize + i * 10;
            write!(f, "{}", position)?;
            for _ in 0..(max_length - position.to_string().len()) {
                write!(f, " ")?;
            }
            write!(f, "│ ")?;

            for base16 in chunk.iter() {
                write!(f, "{:>2}", base16)?;
                write!(f, " ")?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl UpperHex for PiViewerBase16 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let max_length = (self.start + self.buffer.len() as u64).to_string().len();

        for (i, chunk) in self.buffer.chunks(16).enumerate() {
            let position = self.start as usize + i * 16;
            write!(f, "{}", position)?;
            for _ in 0..(max_length - position.to_string().len()) {
                write!(f, " ")?;
            }
            write!(f, "│ ")?;

            for (j, base16) in chunk.iter().enumerate() {
                write!(f, "{:02X}", base16)?;
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

impl LowerHex for PiViewerBase16 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let max_length = (self.start + self.buffer.len() as u64).to_string().len();

        for (i, chunk) in self.buffer.chunks(16).enumerate() {
            let position = self.start as usize + i * 16;
            write!(f, "{}", position)?;
            for _ in 0..(max_length - position.to_string().len()) {
                write!(f, " ")?;
            }
            write!(f, "│ ")?;

            for (j, base16) in chunk.iter().enumerate() {
                write!(f, "{:02x}", base16)?;
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
