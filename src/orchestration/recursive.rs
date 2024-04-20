use crate::traits::{Factorize, PrimalityTest, PrimeFactorization};
use bnum::types::U512;
use num_integer::Integer;
use std::marker::PhantomData;

pub struct RecursivePrimeFactorization<F, P>
where
    F: Factorize,
    P: PrimalityTest,
{
    max_successive_fails: usize,
    _factorizer: PhantomData<F>,
    _prime_tester: PhantomData<P>,
}

impl<Factorizer, PrimeTester> PrimeFactorization
    for RecursivePrimeFactorization<Factorizer, PrimeTester>
where
    Factorizer: Factorize,
    PrimeTester: PrimalityTest,
{
    fn prime_factorization(n: &U512) -> Vec<U512> {
        let max_successive_failures = 100;
        Self::new(max_successive_failures).recursive_factorization(*n)
    }
}

impl<Factorizer, PrimeTester> RecursivePrimeFactorization<Factorizer, PrimeTester>
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

    fn recursive_factorization(&self, mut n: U512) -> Vec<U512> {
        let mut factors = vec![];
        while n.is_even() {
            factors.push(U512::TWO);
            n /= &U512::TWO;
        }
        self.recursion_step(n, &mut factors, 0);
        factors
    }

    fn recursion_step(&self, n: U512, factors: &mut Vec<U512>, retried: usize) {
        if retried == self.max_successive_fails {
            panic![
                "Failed to find factor after {0} succesive attempts",
                self.max_successive_fails
            ]
        }
        if n <= U512::ONE {
            return;
        }
        if PrimeTester::is_prime(&n) {
            factors.push(n);
            return;
        }
        match self.classify_factor(Factorizer::factorize(&n), &n) {
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

    fn classify_factor(&self, factor: U512, n: &U512) -> DivisorOfN {
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
    Prime(U512),
    Composite(U512),
    Trivial(U512),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_framework::prime_factorization::CheckTestBuilder;

    struct FakePrimeTester;

    impl PrimalityTest for FakePrimeTester {
        fn is_prime(n: &U512) -> bool {
            [2, 3, 5].contains(&n.to_str_radix(10).parse().unwrap())
        }
    }

    struct FakeFactorizer;

    impl Factorize for FakeFactorizer {
        fn factorize(n: &U512) -> U512 {
            if n.is_even() {
                return U512::TWO;
            }
            if n % U512::THREE == U512::ZERO {
                return U512::THREE;
            }
            if n % U512::FIVE == U512::ZERO {
                return U512::FIVE;
            }
            *n
        }
    }

    type MyTestOrchestrator = RecursivePrimeFactorization<FakeFactorizer, FakePrimeTester>;

    #[test]
    fn single_prime() {
        CheckTestBuilder::new()
            .case(3, &[3])
            .build::<MyTestOrchestrator>()
            .check_cases()
    }

    #[test]
    fn composite_power_of_2() {
        CheckTestBuilder::new()
            .case(8, &[2; 3])
            .build::<MyTestOrchestrator>()
            .check_cases()
    }

    #[test]
    fn odd_composite() {
        CheckTestBuilder::new()
            .case(15, &[3, 5])
            .build::<MyTestOrchestrator>()
            .check_cases()
    }

    #[test]
    fn even_composite() {
        CheckTestBuilder::new()
            .case(30, &[2, 3, 5])
            .build::<MyTestOrchestrator>()
            .check_cases()
    }

    #[test]
    #[should_panic]
    fn fails_to_find_factor() {
        CheckTestBuilder::new()
            .case(49, &[7; 2])
            .build::<MyTestOrchestrator>()
            .check_cases()
    }
}
