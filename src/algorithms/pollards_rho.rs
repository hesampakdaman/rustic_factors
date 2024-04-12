mod utils;

use crate::traits::Factorize;
use bnum::types::U512;
use num_integer::Integer;
use rustic_factors_derive::RecursivePrimeFactorization;

#[derive(RecursivePrimeFactorization)]
pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(n: &U512) -> U512 {
        let init = U512::from(2u8);
        let pseudorandom_fn = utils::generate_pseudorandom_fn(n);
        let finished = move |x: &U512, y: &U512| x.abs_diff(*y).gcd(n) != U512::from(1u8);
        let (tortoise, hare) = utils::floyds_cycle_detection(init, &pseudorandom_fn, &finished);
        hare.abs_diff(tortoise).gcd(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn composites() {
        test_utils::prime_factorization::TestBuilder::<PollardsRho>::new()
            .case(8051, &[83, 97])
            .case(15, &[3, 5])
            .case(4096, &[2; 12])
            .build()
            .check()
    }

    #[test]
    fn primes() {
        test_utils::prime_factorization::TestBuilder::<PollardsRho>::new()
            .case(3, &[3])
            .case(5, &[5])
            .case(19, &[19])
            .case(29, &[29])
            .case(37, &[37])
            .build()
            .check()
    }
}
