mod composite_evidence;
mod utils;

use self::composite_evidence::CompositeEvidence;
use crate::traits::{Command, PrimalityTest};
use bnum::types::U512;
use num_integer::Integer;

pub struct MillerRabin;

impl Command for MillerRabin {
    fn run(&self, n: &U512) -> String {
        if MillerRabin::is_prime(n) {
            format!("{} is prime", n)
        } else {
            format!("{} is composite", n)
        }
    }
}

impl PrimalityTest for MillerRabin {
    fn is_prime(p: &U512) -> bool {
        if p == &U512::TWO || p == &U512::THREE {
            return true;
        }
        if p < &U512::TWO || p.is_multiple_of(&U512::TWO) {
            return false;
        }
        miller_rabin(p, 50)
    }
}

fn miller_rabin(p: &U512, trials: usize) -> bool {
    let evidence = CompositeEvidence::new(p);
    let likely_prime = |witness| !evidence.witnessed_by(&witness);
    utils::RandomIntegers::new(U512::TWO..p - U512::ONE)
        .take(trials)
        .all(likely_prime)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(p: u32, expected: bool) {
        assert_eq!(
            MillerRabin::is_prime(&U512::from(p)),
            expected,
            "Test failed for {}",
            p
        )
    }

    #[test]
    fn test_prime_numbers() {
        let primes = [3, 5, 7, 11, 104729, 6700417, 2u32.pow(31) - 1];
        for prime in primes {
            check(prime, true);
        }
    }

    #[test]
    fn test_composite_numbers() {
        for composite in [4, 15, 35, 49, 1001] {
            check(composite, false);
        }
    }

    #[test]
    fn test_carmichael_numbers() {
        let carmichaels = [
            561, 1105, 1729, 2465, 2821, 6601, 8911, 10585, 15841, 29341, 41041, 46657, 52633,
            62745, 63973, 75361, 101101, 115921, 126217, 162401, 172081, 188461, 252601, 278545,
            294409, 314821, 334153, 340561, 399001, 410041, 449065, 488881, 512461,
        ];
        for carmichael in carmichaels {
            check(carmichael, false);
        }
    }
}
