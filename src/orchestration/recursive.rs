use crate::traits::{Factorize, PrimalityTest, PrimeFactorization};
use std::marker::PhantomData;

pub struct FactorizeRecursiveWith<F, P>
where
    F: Factorize,
    P: PrimalityTest,
{
    _factorizer: PhantomData<F>,
    _prime_tester: PhantomData<P>,
}

impl<Factorizer, PrimeTester> PrimeFactorization
    for FactorizeRecursiveWith<Factorizer, PrimeTester>
where
    Factorizer: Factorize,
    PrimeTester: PrimalityTest,
{
    fn prime_factorization(n: u128) -> Vec<u128> {
        Self::new().recursive_factorization(n)
    }
}

impl<Factorizer, PrimeTester> FactorizeRecursiveWith<Factorizer, PrimeTester>
where
    Factorizer: Factorize,
    PrimeTester: PrimalityTest,
{
    fn new() -> Self {
        Self {
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
        self.recursion_step(n, &mut factors);
        factors
    }

    fn recursion_step(&self, n: u128, factors: &mut Vec<u128>) {
        if n <= 1 {
            return;
        }
        match self.classify_factor(Factorizer::factorize(n), n) {
            DivisorOfN::Trivial(_) => self.recursion_step(n, factors),
            DivisorOfN::Prime(p) => {
                factors.push(p);
                self.recursion_step(n / p, factors)
            }
            DivisorOfN::Composite(d) => {
                self.recursion_step(n / d, factors);
                self.recursion_step(d, factors);
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
