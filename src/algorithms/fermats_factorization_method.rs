use crate::traits::Factorize;
use bnum::types::U512;
use num_integer::Roots;
use rustic_factors_derive::RecursivePrimeFactorization;

#[derive(RecursivePrimeFactorization)]
pub struct FermatsFactorizationMethod;

impl Factorize for FermatsFactorizationMethod {
    fn factorize(n: &U512) -> U512 {
        let mut a = ceil_sqrt(n);
        let mut b2 = a * a - n;
        while !is_perfect_square(&b2) {
            a += U512::ONE;
            b2 = a * a - n;
        }
        a + b2.sqrt()
    }
}

fn ceil_sqrt(n: &U512) -> U512 {
    if is_perfect_square(n) {
        n.sqrt()
    } else {
        n.sqrt() + U512::ONE
    }
}

fn is_perfect_square(n: &U512) -> bool {
    let sqrt = n.sqrt();
    sqrt * sqrt == *n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_framework::prime_factorization::CheckTestBuilder;

    #[test]
    fn composites() {
        CheckTestBuilder::new()
            .case(5959, &[59, 101])
            .case(12345, &[3, 5, 823])
            .case(102030, &[2, 3, 5, 19, 179])
            .build::<FermatsFactorizationMethod>()
            .check_cases()
    }

    #[test]
    fn primes() {
        CheckTestBuilder::new()
            .case(409, &[409])
            .case(881, &[881])
            .case(1021, &[1021])
            .case(4001, &[4001])
            .case(5003, &[5003])
            .case(9001, &[9001])
            .build::<FermatsFactorizationMethod>()
            .check_cases()
    }
}
