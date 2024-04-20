use bnum::types::U256;
use num_integer::Integer;
use rand::Rng;
use std::ops::Range;

pub struct RandomIntegers {
    range: Range<U256>,
}

impl RandomIntegers {
    pub fn new(range: Range<U256>) -> Self {
        Self { range }
    }
}

impl Iterator for RandomIntegers {
    type Item = U256;

    fn next(&mut self) -> Option<Self::Item> {
        Some(rand::thread_rng().gen_range(self.range.clone()))
    }
}

pub fn highest_power_of_2_divisor(base: &U256) -> u32 {
    let mut exp = 0;
    let mut base = *base;
    while base.is_even() {
        exp += 1;
        base /= U256::TWO;
    }
    exp
}
