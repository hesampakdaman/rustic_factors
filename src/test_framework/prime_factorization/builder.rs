use super::check_test::CheckTest;
use crate::traits::PrimeFactorization;
use bnum::types::U512;

type Factors = Vec<U512>;
type TestCase = (U512, Factors);
pub struct CheckTestBuilder {
    cases: Vec<TestCase>,
}

impl CheckTestBuilder {
    pub fn new() -> Self {
        Self { cases: Vec::new() }
    }

    pub fn case(mut self, n: u128, factors: &[u128]) -> Self {
        let n_u512 = U512::from(n);
        let factors_u512 = factors.iter().map(|&f| U512::from(f)).collect();
        self.cases.push((n_u512, factors_u512));
        self
    }

    pub fn build<F: PrimeFactorization>(self) -> CheckTest<F> {
        CheckTest::<F>::new(self.cases)
    }
}

impl Default for CheckTestBuilder {
    fn default() -> Self {
        Self::new()
            .case(2, &[2])
            .case(3, &[3])
            .case(60, &[2, 2, 3, 5])
            .case(101, &[101])
            .case(121, &[11, 11])
            .case(143, &[11, 13])
            .case(221, &[13, 17])
            .case(396, &[2, 2, 3, 3, 11])
            .case(729, &[3; 6])
            .case(510, &[2, 3, 5, 17])
            .case(897, &[3, 13, 23])
    }
}
