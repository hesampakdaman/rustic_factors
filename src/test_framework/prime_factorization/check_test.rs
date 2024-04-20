use crate::traits::PrimeFactorization;
use bnum::types::U256;
use std::marker::PhantomData;

type TestCase = (U256, Vec<U256>);

pub struct CheckTest<F: PrimeFactorization> {
    cases: Vec<TestCase>,
    _maker: PhantomData<F>,
}

impl<F: PrimeFactorization> CheckTest<F> {
    pub fn new(cases: Vec<TestCase>) -> Self {
        Self {
            cases,
            _maker: PhantomData,
        }
    }

    pub fn check_cases(self) {
        for (n, factors) in &self.cases {
            Self::check(n, &factors);
        }
    }

    pub fn check(n: &U256, expected: &[U256]) {
        let mut actual = F::prime_factorization(&n);
        actual.sort_unstable();
        assert_eq!(actual, expected, "Test failed for n = {}", n);
    }
}
