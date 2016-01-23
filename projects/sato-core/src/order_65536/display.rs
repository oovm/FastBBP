use super::*;

impl Display for PiViewer16 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let max_length = (self.start as usize + self.buffer.len()).length();

        for (i, chunk) in self.buffer.chunks(10).enumerate() {
            let position = self.start as usize + i * 10;
            write!(f, "{}", position)?;
            for _ in 0..(max_length - position.length()) {
                write!(f, " ")?;
            }
            write!(f, "│ ")?;

            for base16 in chunk.iter() {
                write!(f, "{:>3}", base16)?;
                write!(f, " ")?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl UpperHex for PiViewer16 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&HexViewer16 { lower: false, start: self.start, buffer: &self.buffer }, f)
    }
}

impl LowerHex for PiViewer16 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&HexViewer16 { lower: true, start: self.start, buffer: &self.buffer }, f)
    }
}
