mod utils;

use crate::orchestration;
use crate::primality_test::MillerRabin;
use crate::traits::Factorize;
use crate::PrimeFactorization;

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(n: u128) -> u128 {
        let init = 2;
        let psudorandom_fn = utils::generate_psudeorandom_fn(n);
        let finished = |x: u128, y: u128| utils::gcd(x.abs_diff(y), n) != 1;
        let (tortoise, hare) = utils::floyds_cycle_detection(init, &psudorandom_fn, &finished);
        utils::gcd(tortoise.abs_diff(hare), n)
    }
}

impl PrimeFactorization for PollardsRho {
    fn prime_factorization(n: u128) -> Vec<u128> {
        orchestration::FactorizeRecursiveWith::<Self, MillerRabin>::prime_factorization(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composites() {
        let test_cases = [(8051, vec![83, 97]), (15, vec![3, 5]), (4096, vec![2; 12])];
        for (n, expected) in test_cases {
            let mut actual = PollardsRho::prime_factorization(n);
            actual.sort_unstable();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn primes() {
        let primes = [3, 5, 19, 29, 37, 7027, 13037];
        for p in primes {
            let actual = PollardsRho::prime_factorization(p);
            assert_eq!(actual, vec![p]);
        }
    }
}
