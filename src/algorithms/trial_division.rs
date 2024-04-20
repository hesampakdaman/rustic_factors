use crate::PrimeFactorization;
use bnum::types::U256;
use num_integer::Integer;
use num_traits::One;
use rustic_factors_derive::FactorizationCommand;

#[derive(FactorizationCommand)]
pub struct TrialDivision;

impl PrimeFactorization for TrialDivision {
    fn prime_factorization(n: &U256) -> Vec<U256> {
        if n <= &U256::one() {
            return vec![*n];
        }
        trial_div(*n)
    }
}

fn trial_div(mut n: U256) -> Vec<U256> {
    let mut factors = vec![];
    let divisors = DivisorCandidates::new();
    for d in divisors {
        if n < d.pow(2) {
            break;
        }
        while n.is_multiple_of(&d) {
            n /= &d;
            factors.push(d);
        }
    }
    if is_still_undivided(&n) {
        factors.push(n);
    }
    factors
}

fn is_still_undivided(n: &U256) -> bool {
    !n.is_one()
}

struct DivisorCandidates {
    current: U256,
}

impl DivisorCandidates {
    fn new() -> Self {
        DivisorCandidates {
            current: U256::from(2u8),
        }
    }
}

impl Iterator for DivisorCandidates {
    type Item = U256;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.current;
        self.current = if self.current == U256::from(2u8) {
            self.current + U256::from(1u8)
        } else {
            self.current + U256::from(2u8)
        };
        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_framework::prime_factorization::CheckTestBuilder;

    #[test]
    fn prime() {
        CheckTestBuilder::new()
            .case(13, &[13])
            .build::<TrialDivision>()
            .check_cases()
    }

    #[test]
    fn composite() {
        CheckTestBuilder::new()
            .case(12, &[2, 2, 3])
            .build::<TrialDivision>()
            .check_cases()
    }
}
