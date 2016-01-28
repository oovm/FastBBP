use core::fmt::{Display, Formatter};
use dashu::{base::SquareRoot, float::DBig, integer::IBig};
use std::ops::{Add, Div, Mul};

/// Ramanujan Level1
#[derive(Copy, Clone, Debug)]
pub struct RamanujanL1 {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: u64,
    f: u64,
}

impl Display for RamanujanL1 {
    // (13591409-Sum[Product[((6 j-1) (2 j-1) (6 j-5))/(10939058860032000 j^3),{j,1,k}]*(545140134 k+13591409),{k,1,Infinity}])/(426880 Sqrt[10005])
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "({}-Sum[Product[((6 j-1) (2 j-1) (6 j-5))/({} j^3),{{j,1,k}}]}}/({} k+{}),{{k,1,Infinity}}])/({} Sqrt[{}])",
            self.c, self.d, self.e, self.f, self.a, self.b
        )
    }
}

impl RamanujanL1 {
    pub const J163: Self = Self { a: 426880, b: 10005, c: 13591409, d: 10939058860032000, e: 545140134, f: 13591409 };
}

fn binary_split(j: i64, i: i64) -> (IBig, IBig, IBig) {
    if i == j + 1 {
        let pab = IBig::from(-(6 * j - 5) * (2 * j - 1) * (6 * j - 1));
        let qab = IBig::from(j).pow(3).mul(10939058860032000u64);
        let rab = IBig::from(545140134 * j + 13591409).mul(&pab);
        (pab, qab, rab)
    }
    else {
        let m = (j + i) / 2;
        let (pam, qam, ram) = binary_split(j, m);
        let (pmb, qmb, rmb) = binary_split(m, i);
        let pab = pmb.mul(&pam);
        let qab = qam.mul(&qmb);
        let rab = ram.mul(qmb) + pam * rmb;
        (pab, qab, rab)
    }
}

impl RamanujanL1 {
    pub fn run(&self, iterators: usize) -> DBig {
        DBig::default()
    }
}

fn chudnovsky_iter(iterators: usize) -> DBig {
    // The fastest such formula gives approximately 14 significant figures
    const RELAXATION: usize = 15;
    let (_, q1n, r1n) = binary_split(1, iterators as i64);
    let d = DBig::from(13591409u64).mul(&q1n).add(r1n).with_precision(RELAXATION * iterators).value();
    let n = DBig::from(q1n.mul(426880u64)).with_precision(RELAXATION * iterators).value();
    let sqrt = DBig::from(10005).with_precision(RELAXATION * iterators).value().sqrt();
    n.mul(sqrt).div(d)
}

pub fn chudnovsky(iterators: usize) -> DBig {
    RamanujanL1::J163.run(iterators)
}

#[test]
fn main() {
    println!("{}", chudnovsky_iter(10))
}
