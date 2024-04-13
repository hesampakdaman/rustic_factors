pub mod prime_factorization {
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
            CheckTest::<F>::new(self.cases)
        }
    }

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

        pub fn check(&self) {
            for (n, factors) in &self.cases {
                self.check_case(*n, factors);
            }
        }

        fn check_case(&self, n: u128, factors: &[u128]) {
            let expected: Vec<U512> = factors.iter().map(|&d| U512::from(d)).collect();
            let mut actual = F::prime_factorization(&U512::from(n));
            actual.sort_unstable();
            assert_eq!(actual, expected, "Test failed for n = {}", n);
        }
    }
}
