use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use std::ops::Range;

pub struct RandomIntegers {
    lo: BigUint,
    hi: BigUint,
}

impl RandomIntegers {
    pub fn new(range: Range<BigUint>) -> Self {
        Self {
            lo: range.start,
            hi: range.end,
        }
    }
}

impl Iterator for RandomIntegers {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        Some(rand::thread_rng().gen_biguint_range(&self.lo, &self.hi))
    }
}

pub fn highest_power_of_2_divisor(base: &BigUint) -> u32 {
    let mut exp = 0;
    let mut base = base.clone();
    while base.is_even() {
        exp += 1;
        base /= 2u8;
    }
    exp
}
