use super::*;

impl Display for PiViewerBase256 {
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
                write!(f, "{:>3}", base16)?;
                write!(f, " ")?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub(crate) struct HexViewer8<'i> {
    lower: bool,
    start: u64,
    buffer: &'i [u8],
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


impl UpperHex for PiViewerBase256 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&HexViewer8 {
            lower: false,
            start: self.start,
            buffer: &self.buffer,
        }, f)
    }
}

impl LowerHex for PiViewerBase256 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&HexViewer8 {
            lower: true,
            start: self.start,
            buffer: &self.buffer,
        }, f)
    }
}
