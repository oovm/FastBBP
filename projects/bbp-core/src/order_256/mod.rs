use crate::helpers::pow_mod;

const ORDER256: u64 = 256;


fn Σ(digit: u64, i: u64) -> f64 {
    let mut s: f64 = 0.0;
    let mut denom = i;
    for k in 0..digit + 1 {
        let r = pow_mod(ORDER256, digit - k, denom);
        s = Δ(s + r as f64 / denom as f64);
        denom += 16;
    }
    let mut num = 1.0 / ORDER256 as f64;
    while num / denom as f64 > f64::EPSILON {
        s += num / denom as f64;
        num /= ORDER256 as f64;
        denom += 16;
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
pub fn bbp256(digit: u64) -> u8 {
    let r = Δ(256.0 * Σ(digit, 1) - (128.0 * Σ(digit, 4)) - 64.0 * Σ(digit, 5) - 64.0 * Σ(digit, 6) + 16.0 * Σ(digit, 9) - (8.0 * Σ(digit, 12)) - 4.0 * Σ(digit, 13) - 4.0 * Σ(digit, 14));
    store_u8((ORDER256 as f64 * r).floor())
}

