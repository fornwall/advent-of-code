/// From https://docs.rs/mod_exp/1.0.1/mod_exp, specialised to i128.
pub fn mod_exp(base: i128, exponent: i128, modulus: i128) -> i128 {
    const ONE: i128 = 1;
    const TWO: i128 = 2;
    const ZERO: i128 = 0;
    const MAX: i128 = i128::MAX;

    assert!((modulus - ONE) < (MAX / (modulus - ONE)));

    let mut result = ONE;
    let mut base = base % modulus;
    let mut exponent = exponent;

    loop {
        if exponent <= ZERO {
            break;
        }

        if exponent % TWO == ONE {
            result = (result * base) % modulus;
        }

        exponent >>= ONE;
        base = (base * base) % modulus;
    }

    result
}
