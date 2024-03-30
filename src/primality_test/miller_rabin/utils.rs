use rand::Rng;
use std::ops::Range;

pub struct RandomIntegers {
    range: Range<u128>,
}

impl RandomIntegers {
    pub fn new(range: Range<u128>) -> Self {
        Self { range }
    }
}

impl Iterator for RandomIntegers {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        Some(rand::thread_rng().gen_range(self.range.clone()))
    }
}

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

pub fn is_nontrivial_sqrt_of_1(solution: u128, number: u128) -> bool {
    let squared = (solution * solution) % number;
    squared == 1 && solution != 1 && solution != number - 1
}
