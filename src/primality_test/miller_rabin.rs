mod utils;
mod witnessed;

use rand::Rng;
use std::ops::Range;

use crate::traits::PrimalityTest;
use witnessed::Witnessed;

pub struct MillerRabin;

impl PrimalityTest for MillerRabin {
    fn is_prime(&self, p: u128) -> bool {
        if p == 2 {
            return true;
        }
        if p < 2 || p % 2 == 0 {
            return false;
        }
        miller_rabin_test(p, 10)
    }
}

fn miller_rabin_test(p: u128, trials: usize) -> bool {
    let witnessed = Witnessed::new(p);
    let is_no_witness_for_p = |a| !witnessed.by(a);
    RandomIntegers::new(1..p)
        .take(trials)
        .all(is_no_witness_for_p)
}

struct RandomIntegers {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_numbers() {
        let primes = [
            3,
            5,
            7,
            11,
            13,
            17,
            104729,
            6700417,
            2147483647,
            32416190071,
            99194853094755497,
            2305843009213693951,
        ];
        for &prime in primes.iter() {
            assert!(MillerRabin.is_prime(prime), "Failed on prime {prime}");
        }
    }

    #[test]
    fn test_composite_numbers() {
        for &composite in [4, 6, 8, 9, 10, 12, 14, 15, 1001, 1024].iter() {
            assert!(
                !MillerRabin.is_prime(composite),
                "Failed on composite {composite}"
            );
        }
    }

    #[test]
    fn test_carmichael_numbers() {
        let carmichaels = [
            561, 1105, 1729, 2465, 2821, 6601, 8911, 10585, 15841, 29341, 41041, 46657, 52633,
            62745, 63973, 75361, 101101, 115921, 126217, 162401, 172081, 188461, 252601, 278545,
            294409, 314821, 334153, 340561, 399001, 410041, 449065, 488881, 512461,
        ];
        for &carmichael in carmichaels.iter() {
            assert!(
                !MillerRabin.is_prime(carmichael),
                "Failed on Carmichael number {carmichael}"
            );
        }
    }
}
