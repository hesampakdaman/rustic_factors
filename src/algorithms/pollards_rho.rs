mod utils;

use crate::orchestration;
use crate::primality_test::MillerRabin;
use crate::traits::Factorize;
use crate::PrimeFactorization;
use bnum::types::U512;
use num_integer::Integer;

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(n: &U512) -> U512 {
        let init = U512::from(2u8);
        let pseudorandom_fn = utils::generate_pseudorandom_fn(&n);
        let finished = move |x: &U512, y: &U512| x.abs_diff(*y).gcd(&n) != U512::from(1u8);
        let (tortoise, hare) = utils::floyds_cycle_detection(init, &pseudorandom_fn, &finished);
        hare.abs_diff(tortoise).gcd(&n)
    }
}

impl PrimeFactorization for PollardsRho {
    fn prime_factorization(n: &U512) -> Vec<U512> {
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
