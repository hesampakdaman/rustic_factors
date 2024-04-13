use super::check_test::CheckTest;
use crate::traits::PrimeFactorization;
use bnum::types::U512;
use std::marker::PhantomData;

type Factors = Vec<U512>;
type TestCase = (U512, Factors);
pub struct CheckTestBuilder<F: PrimeFactorization> {
    cases: Vec<TestCase>,
    _marker: PhantomData<F>,
}

impl<F: PrimeFactorization> CheckTestBuilder<F> {
    pub fn new() -> Self {
        Self {
            cases: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn case(mut self, n: u128, factors: &[u128]) -> Self {
        let n_u512 = U512::from(n);
        let factors_u512 = factors.iter().map(|&f| U512::from(f)).collect();
        self.cases.push((n_u512, factors_u512));
        self
    }

    pub fn build(self) -> CheckTest<F> {
        CheckTest::<F>::new(self.cases)
    }
}
