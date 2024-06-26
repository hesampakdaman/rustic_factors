mod utils;

use crate::traits::{Factorize, PrimeFactorization};
use bnum::types::U512;
use num_integer::Integer;
use rustic_factors_derive::{FactorizationCommand, RecursivePrimeFactorization};

#[derive(FactorizationCommand, RecursivePrimeFactorization)]
pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(n: &U512) -> U512 {
        let init = U512::TWO;
        let pseudorandom_fn = utils::generate_pseudorandom_fn(n);
        let finished = move |x: &U512, y: &U512| x.abs_diff(*y).gcd(n) != U512::ONE;
        let (tortoise, hare) = utils::floyds_cycle_detection(init, &pseudorandom_fn, &finished);
        hare.abs_diff(tortoise).gcd(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_framework::prime_factorization::CheckTestBuilder;

    #[test]
    fn default() {
        CheckTestBuilder::default()
            .build::<PollardsRho>()
            .check_cases()
    }

    #[test]
    fn composites() {
        CheckTestBuilder::new()
            .case(8051, &[83, 97])
            .case(15, &[3, 5])
            .case(4096, &[2; 12])
            .build::<PollardsRho>()
            .check_cases()
    }

    #[test]
    fn primes() {
        CheckTestBuilder::new()
            .case(3, &[3])
            .case(5, &[5])
            .case(19, &[19])
            .case(29, &[29])
            .case(37, &[37])
            .build::<PollardsRho>()
            .check_cases()
    }
}
