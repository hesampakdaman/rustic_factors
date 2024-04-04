use crate::traits::{Factorize, PrimalityTest, PrimeFactorization};
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
    fn prime_factorization(n: u128) -> Vec<u128> {
        let max_successive_failures = 5;
        Self::new(max_successive_failures).recursive_factorization(n)
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

    fn recursive_factorization(&self, mut n: u128) -> Vec<u128> {
        let mut factors = vec![];
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }
        self.recursion_step(n, &mut factors, 0);
        factors
    }

    fn recursion_step(&self, n: u128, factors: &mut Vec<u128>, retried: usize) {
        if retried == self.max_successive_fails {
            panic![
                "Failed to find factor after {0} succesive attempts",
                self.max_successive_fails
            ]
        }
        if n <= 1 {
            return;
        }
        match self.classify_factor(Factorizer::factorize(n), n) {
            DivisorOfN::Trivial(_) => self.recursion_step(n, factors, retried + 1),
            DivisorOfN::Prime(p) => {
                factors.push(p);
                self.recursion_step(n / p, factors, 0);
            }
            DivisorOfN::Composite(d) => {
                self.recursion_step(n / d, factors, 0);
                self.recursion_step(d, factors, 0);
            }
        }
    }

    fn classify_factor(&self, factor: u128, n: u128) -> DivisorOfN {
        if PrimeTester::is_prime(factor) {
            return DivisorOfN::Prime(factor);
        }
        if factor == 1 || factor == n {
            return DivisorOfN::Trivial(factor);
        }
        DivisorOfN::Composite(factor)
    }
}

enum DivisorOfN {
    Prime(u128),
    Composite(u128),
    Trivial(u128),
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakePrimeTester;

    impl PrimalityTest for FakePrimeTester {
        fn is_prime(n: u128) -> bool {
            [2, 3, 5].contains(&n)
        }
    }

    struct FakeFactorizer;

    impl Factorize for FakeFactorizer {
        fn factorize(n: u128) -> u128 {
            if n % 2 == 0 {
                return 2;
            }
            if n % 3 == 0 {
                return 3;
            }
            if n % 5 == 0 {
                return 5;
            }
            n
        }
    }

    type MyTestOrchestrator = FactorizeRecursiveWith<FakeFactorizer, FakePrimeTester>;

    #[test]
    fn single_prime() {
        assert_eq!(MyTestOrchestrator::prime_factorization(3), vec![3]);
    }

    #[test]
    fn composite_power_of_2() {
        assert_eq!(MyTestOrchestrator::prime_factorization(8), vec![2; 3]);
    }

    #[test]
    fn odd_composite() {
        assert_eq!(MyTestOrchestrator::prime_factorization(15), vec![3, 5]);
    }

    #[test]
    fn even_composite() {
        assert_eq!(MyTestOrchestrator::prime_factorization(30), vec![2, 3, 5]);
    }

    #[test]
    #[should_panic]
    fn fails_to_find_factor() {
        MyTestOrchestrator::prime_factorization(49);
    }
}
