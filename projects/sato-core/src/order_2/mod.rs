use core::fmt::{Display, LowerHex, UpperHex};
use dashu::float::DBig;

mod display;

/// Ramanujan Level2 Series
#[derive(Copy, Clone, Debug, Default)]
pub struct RamanujanL2 {
    a: usize,
}

impl RamanujanL2 {
    pub const J58: Self = Self { a: 0 };
}

impl RamanujanL2 {
    pub fn run(&self, iterators: usize) -> DBig {
        DBig::default()
    }
}

pub fn ramanujan(iterators: usize) -> DBig {
    RamanujanL2::J58.run(iterators)
}
