mod composite_evidence;
mod utils;

use self::composite_evidence::CompositeEvidence;
use crate::traits::PrimalityTest;

pub struct MillerRabin;

impl PrimalityTest for MillerRabin {
    fn is_prime(p: u128) -> bool {
        if p == 2 || p == 3 {
            return true;
        }
        if p < 2 || p % 2 == 0 {
            return false;
        }
        miller_rabin(p, 10)
    }
}

fn miller_rabin(p: u128, trials: usize) -> bool {
    let evidence = CompositeEvidence::new(p);
    let likely_prime = |witness| !evidence.witnessed_by(witness);
    utils::RandomIntegers::new(2..p - 1)
        .take(trials)
        .all(likely_prime)
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
            104729,
            6700417,
            2u128.pow(31) - 1,
            2u128.pow(61) - 1,
        ];
        for prime in primes {
            assert!(MillerRabin::is_prime(prime), "Failed on prime {prime}");
        }
    }

    #[test]
    fn test_composite_numbers() {
        for composite in [4, 15, 35, 49, 1001] {
            assert!(
                !MillerRabin::is_prime(composite),
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
        for carmichael in carmichaels {
            assert!(
                !MillerRabin::is_prime(carmichael),
                "Failed on Carmichael number {carmichael}"
            );
        }
    }
}
