use crate::orchestration;
use crate::primality_test::MillerRabin;
use crate::traits::Factorize;
use crate::PrimeFactorization;
use bnum::types::U512;
use num_integer::Roots;

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

impl PrimeFactorization for FermatsFactorizationMethod {
    fn prime_factorization(n: &U512) -> Vec<U512> {
        orchestration::FactorizeRecursiveWith::<Self, MillerRabin>::prime_factorization(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utils::check_factorization;

    fn check(n: u32, factors: &[u32]) {
        check_factorization::<FermatsFactorizationMethod>(n, factors);
    }

    #[test]
    fn composites() {
        let test_cases = [
            (5959, vec![59, 101]),
            (12345, vec![3, 5, 823]),
            (102030, vec![2, 3, 5, 19, 179]),
        ];
        for (n, expected) in test_cases {
            check(n, &expected);
        }
    }

    #[test]
    fn primes() {
        let primes = [409, 881, 1021, 4001, 5003, 9001];
        for p in primes {
            check(p, &[p]);
        }
    }
}
