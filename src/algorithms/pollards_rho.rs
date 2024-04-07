mod utils;

use crate::orchestration;
use crate::primality_test::MillerRabin;
use crate::traits::Factorize;
use crate::PrimeFactorization;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed};

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(n: &BigInt) -> BigInt {
        let init = BigInt::from(2);
        let psudorandom_fn = utils::generate_pseudorandom_fn(n);
        let finished = move |x: &BigInt, y: &BigInt| (x - y).abs().gcd(n) != BigInt::one();
        let (tortoise, hare) = utils::floyds_cycle_detection(init, &psudorandom_fn, &finished);
        (hare - tortoise).abs().gcd(n)
    }
}

impl PrimeFactorization for PollardsRho {
    fn prime_factorization(n: &BigInt) -> Vec<BigInt> {
        orchestration::FactorizeRecursiveWith::<Self, MillerRabin>::prime_factorization(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utils::check_factorization;

    fn check(n: u32, factors: &[u32]) {
        check_factorization::<PollardsRho>(n, factors);
    }

    #[test]
    fn composites() {
        let test_cases = [(8051, vec![83, 97]), (15, vec![3, 5]), (4096, vec![2; 12])];
        for (n, expected) in test_cases {
            check(n, &expected);
        }
    }

    #[test]
    fn primes() {
        let primes = [3, 5, 19, 29, 37];
        for p in primes {
            check(p, &[p])
        }
    }
}
