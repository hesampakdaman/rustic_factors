use super::check_test::CheckTest;
use crate::traits::PrimeFactorization;
use bnum::types::U512;
use std::marker::PhantomData;

type Factors = Vec<u128>;
type TestCase = (u128, Factors);
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
        self.cases.push((n, Vec::from(factors)));
        self
    }

    pub fn build(self) -> CheckTest<F> {
        let cases: Vec<(U512, Vec<U512>)> = self
            .cases
            .into_iter()
            .map(|(n, factors)| {
                (
                    U512::from(n),
                    factors.into_iter().map(|f| U512::from(f)).collect(),
                )
            })
            .collect();
        CheckTest::<F>::new(cases)
    }
}
