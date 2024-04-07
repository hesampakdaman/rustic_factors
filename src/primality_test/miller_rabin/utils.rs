use num_bigint::{BigInt, RandBigInt};
use num_integer::Integer;
use std::ops::Range;

pub struct RandomIntegers {
    lo: BigInt,
    hi: BigInt,
}

impl RandomIntegers {
    pub fn new(range: Range<BigInt>) -> Self {
        Self {
            lo: range.start,
            hi: range.end,
        }
    }
}

impl Iterator for RandomIntegers {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        Some(rand::thread_rng().gen_bigint_range(&self.lo, &self.hi))
    }
}

pub fn highest_power_of_2_divisor(base: &BigInt) -> u32 {
    let mut exp = 0;
    let mut base = base.clone();
    while base.is_even() {
        exp += 1;
        base /= 2u8;
    }
    exp
}
