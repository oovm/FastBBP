use core::fmt::{Display, Formatter};
use dashu::{base::SquareRoot, float::DBig, integer::IBig};
use std::ops::{Add, Div, Mul};

/// Ramanujan Level1 Series
#[derive(Copy, Clone, Debug)]
pub struct RamanujanL1 {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
}

impl Display for RamanujanL1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "({}Sqrt[{}])/({}-Sum[Product[(72j^3-108j^2+46j-5)/({}j^3),{{j,1,k}}]/({}k+{}),{{k,1,Infinity}}])",
            self.a, self.b, self.c, self.d, self.e, self.f,
        )
    }
}

impl RamanujanL1 {
    /// let `j = (1 + sqrt(-7)) / 2, tau = ?, a = ?, b = ?`
    pub const J7: Self = Self { a: 426880, b: 10005, c: 13591409, d: 10939058860032000, e: 545140134, f: 13591409 };
    /// let `j = (1 + sqrt(-11)) / 2`
    pub const J11: Self = Self { a: 426880, b: 10005, c: 13591409, d: 10939058860032000, e: 545140134, f: 13591409 };
    /// let `j = (1 + sqrt(-19)) / 2`
    pub const J19: Self = Self { a: 426880, b: 10005, c: 13591409, d: 10939058860032000, e: 545140134, f: 13591409 };
    /// let `j = (1 + sqrt(-43)) / 2`
    pub const J43: Self = Self { a: 426880, b: 10005, c: 13591409, d: 10939058860032000, e: 545140134, f: 13591409 };
    /// let `j = (1 + sqrt(-67)) / 2`
    pub const J67: Self = Self { a: 426880, b: 10005, c: 13591409, d: 10939058860032000, e: 545140134, f: 13591409 };
    /// let `j = (1 + sqrt(-163)) / 2`
    pub const J163: Self = Self { a: 426880, b: 10005, c: 13591409, d: 10939058860032000, e: 545140134, f: 13591409 };
}

impl RamanujanL1 {
    fn binary_split(&self, j: i64, i: i64) -> (IBig, IBig, IBig) {
        if i == j + 1 {
            let pab = IBig::from(-(6 * j - 5) * (2 * j - 1) * (6 * j - 1));
            let qab = IBig::from(j).pow(3).mul(self.d);
            let rab = IBig::from(self.e * j + self.f).mul(&pab);
            (pab, qab, rab)
        }
        else {
            let m = (j + i) / 2;
            let (pam, qam, ram) = self.binary_split(j, m);
            let (pmb, qmb, rmb) = self.binary_split(m, i);
            let pab = pmb.mul(&pam);
            let qab = qam.mul(&qmb);
            let rab = ram.mul(qmb) + pam * rmb;
            (pab, qab, rab)
        }
    }
    pub fn run(&self, iterators: usize) -> DBig {
        // The fastest such formula gives approximately 14 significant figures
        const RELAXATION: usize = 15;
        let (_, q1n, r1n) = self.binary_split(1, iterators as i64);
        let d = DBig::from(self.c).mul(&q1n).add(r1n).with_precision(RELAXATION * iterators).value();
        let n = DBig::from(q1n.mul(self.a)).with_precision(RELAXATION * iterators).value();
        let sqrt = DBig::from(self.b).with_precision(RELAXATION * iterators).value().sqrt();
        n.mul(sqrt).div(d)
    }
}

/// Compute π using chudnovsky's algorithm
///
/// Each iteration gives about 14 digits of effective precision
pub fn chudnovsky(iterators: usize) -> DBig {
    RamanujanL1::J163.run(iterators)
}
