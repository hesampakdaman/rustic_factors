use bnum::types::U512;
use num_integer::Integer;
use rand::Rng;
use std::ops::Range;

pub struct RandomIntegers {
    range: Range<U512>,
}

impl RandomIntegers {
    pub fn new(range: Range<U512>) -> Self {
        Self { range }
    }
}

impl Iterator for RandomIntegers {
    type Item = U512;

    fn next(&mut self) -> Option<Self::Item> {
        Some(rand::thread_rng().gen_range(self.range.clone()))
    }
}

pub fn highest_power_of_2_divisor(base: &U512) -> u32 {
    let mut exp = 0;
    let mut base = *base;
    while base.is_even() {
        exp += 1;
        base /= U512::TWO;
    }
    exp
}
