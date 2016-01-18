fn mod_pow(mut base: u32, mut exp: u32, modulus: u32) -> u32 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn Σ(digit: u32, j: u32) -> f64 {
    let mut s: f64 = 0.0;
    let mut denom = j;
    for k in 0..digit + 1 {
        let r = mod_pow(16, digit - k, denom);
        s = Δ(s + f64::from(r) / f64::from(denom));
        denom += 8u32;
    }
    let mut num = 1.0 / 16.0;
    while num / f64::from(denom) > f64::EPSILON {
        s += num / f64::from(denom);
        num /= 16.0;
        denom += 8u32;
    }
    s.fract()
}

fn Δ(v: f64) -> f64 {
    if v < 0.0 {
        return 1.0 + v.fract() % 1.0;
    }
    v.fract() % 1.0
}

fn f64_to_u8(value: f64) -> u8 {
    let clamped_value = value.round().max(u8::MIN as f64).min(u8::MAX as f64);
    clamped_value as u8
}

fn bbp(digit: u32) -> u8 {
    let r = Δ(4.0 * Σ(digit - 1, 1) - (2.0 * Σ(digit - 1, 4)) - Σ(digit - 1, 5) - Σ(digit - 1, 6));
    f64_to_u8((16.0 * r).floor())
}


#[test]
fn test() {
    println!("BBP[4]: 1th decimal: {:X}", bbp(1));
    println!("BBP[4]: 10th decimal: {:X}", bbp(10));
    println!("BBP[4]: 100th decimal: {:X}", bbp(100));
    println!("BBP[4]: 1000th decimal: {:X}", bbp(1000));
    println!("BBP[4]: 8196th decimal: {:X}", bbp(8196));
    // Overflow
}
