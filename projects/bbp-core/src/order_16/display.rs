use super::*;
use crate::helpers::DecViewer;

impl Display for PiViewer4 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&DecViewer { start: self.start, buffer: &self.buffer }, f)
    }
}

impl UpperHex for PiViewer4 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&HexViewer8 { lower: false, start: self.start, buffer: &self.buffer }, f)
    }
}

impl LowerHex for PiViewer4 {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&HexViewer8 { lower: true, start: self.start, buffer: &self.buffer }, f)
    }
}
