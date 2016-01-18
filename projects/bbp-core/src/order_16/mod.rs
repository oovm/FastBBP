use crate::helpers::pow_mod;

const ORDER16: u64 = 16;


fn Σ(digit: u64, j: u64) -> f64 {
    let mut s: f64 = 0.0;
    let mut denom = j;
    for k in 0..digit + 1 {
        let r = pow_mod(ORDER16, digit - k, denom);
        s = Δ(s + r as f64 / denom as f64);
        denom += 8;
    }
    let mut num = 1.0 / ORDER16 as f64;
    while num / denom as f64 > f64::EPSILON {
        s += num / denom as f64;
        num /= ORDER16 as f64;
        denom += 8;
    }
    s.fract()
}

fn Δ(v: f64) -> f64 {
    if v < 0.0 {
        return 1.0 + v.fract() % 1.0;
    }
    v.fract() % 1.0
}

pub(crate) fn store_u8(value: f64) -> u8 {
    value.round().max(u8::MIN as f64).min(u8::MAX as f64) as u8
}

/// Original [Bailey–Borwein–Plouffe]() formula
pub fn bbp16(digit: u64) -> u8 {
    let r = Δ(4.0 * Σ(digit, 1) - (2.0 * Σ(digit, 4)) - Σ(digit, 5) - Σ(digit, 6));
    store_u8((ORDER16 as f64 * r).floor())
}

