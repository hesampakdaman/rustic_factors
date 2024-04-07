use crate::traits::{Factorize, PrimalityTest, PrimeFactorization};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;
use std::marker::PhantomData;

pub struct FactorizeRecursiveWith<F, P>
where
    F: Factorize,
    P: PrimalityTest,
{
    max_successive_fails: usize,
    _factorizer: PhantomData<F>,
    _prime_tester: PhantomData<P>,
}

impl<Factorizer, PrimeTester> PrimeFactorization for FactorizeRecursiveWith<Factorizer, PrimeTester>
where
    Factorizer: Factorize,
    PrimeTester: PrimalityTest,
{
    fn prime_factorization(n: &BigInt) -> Vec<BigInt> {
        let max_successive_failures = 5;
        Self::new(max_successive_failures).recursive_factorization(n.clone())
    }
}

impl<Factorizer, PrimeTester> FactorizeRecursiveWith<Factorizer, PrimeTester>
where
    Factorizer: Factorize,
    PrimeTester: PrimalityTest,
{
    fn new(max_successive_fails: usize) -> Self {
        Self {
            max_successive_fails,
            _factorizer: PhantomData,
            _prime_tester: PhantomData,
        }
    }

    fn recursive_factorization(&self, mut n: BigInt) -> Vec<BigInt> {
        let mut factors = vec![];
        let two = BigInt::from(2);
        while n.is_even() {
            factors.push(two.clone());
            n /= &two;
        }
        self.recursion_step(n, &mut factors, 0);
        factors
    }

    fn recursion_step(&self, n: BigInt, factors: &mut Vec<BigInt>, retried: usize) {
        if retried == self.max_successive_fails {
            panic![
                "Failed to find factor after {0} succesive attempts",
                self.max_successive_fails
            ]
        }
        if n <= BigInt::one() {
            return;
        }
        match self.classify_factor(Factorizer::factorize(&n), &n) {
            DivisorOfN::Trivial(_) => self.recursion_step(n, factors, retried + 1),
            DivisorOfN::Prime(p) => {
                factors.push(p.clone());
                self.recursion_step(n / p, factors, 0);
            }
            DivisorOfN::Composite(d) => {
                self.recursion_step(n / d.clone(), factors, 0);
                self.recursion_step(d, factors, 0);
            }
        }
    }

    fn classify_factor(&self, factor: BigInt, n: &BigInt) -> DivisorOfN {
        if PrimeTester::is_prime(&factor) {
            return DivisorOfN::Prime(factor);
        }
        if factor.is_one() || &factor == n {
            return DivisorOfN::Trivial(factor);
        }
        DivisorOfN::Composite(factor)
    }
}

enum DivisorOfN {
    Prime(BigInt),
    Composite(BigInt),
    Trivial(BigInt),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utils::check_factorization;
    use num_traits::Zero;

    struct FakePrimeTester;

    impl PrimalityTest for FakePrimeTester {
        fn is_prime(n: &BigInt) -> bool {
            [2, 3, 5].contains(&n.try_into().unwrap())
        }
    }

    struct FakeFactorizer;

    impl Factorize for FakeFactorizer {
        fn factorize(n: &BigInt) -> BigInt {
            if n.is_even() {
                return 2.into();
            }
            if n % 3 == BigInt::zero() {
                return 3.into();
            }
            if n % 5 == BigInt::zero() {
                return 5.into();
            }
            n.to_owned()
        }
    }

    type MyTestOrchestrator = FactorizeRecursiveWith<FakeFactorizer, FakePrimeTester>;

    fn check(n: u32, factors: &[u32]) {
        check_factorization::<MyTestOrchestrator>(n, factors)
    }

    #[test]
    fn single_prime() {
        check(3, &[3])
    }

    #[test]
    fn composite_power_of_2() {
        check(8, &[2; 3])
    }

    #[test]
    fn odd_composite() {
        check(15, &[3, 5])
    }

    #[test]
    fn even_composite() {
        check(30, &[2, 3, 5])
    }

    #[test]
    #[should_panic]
    fn fails_to_find_factor() {
        check(49, &[7, 7])
    }
}
