pub fn modular_exponentiation(base: u128, exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut base = base % modulus;
    let mut exp = exp;
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus; // If the exponent is odd, multiply the result by the base
        }
        exp >>= 1; // (divide by 2, dropping any remainder)
        base = (base * base) % modulus;
    }
    result
}

pub fn highest_power_of_2_divisor(base: u128) -> u32 {
    let mut exp = 0;
    let mut base = base;
    while base % 2 == 0 {
        exp += 1;
        base /= 2;
    }
    exp
}

pub fn fails_fermats_test(x: u128) -> bool {
    x != 1
}
