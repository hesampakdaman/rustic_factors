use crate::PrimeFactorization;
use bnum::types::U512;
use num_integer::Integer;
use rustic_factors_derive::FactorizationCommand;

#[derive(FactorizationCommand)]
pub struct TrialDivision;

impl PrimeFactorization for TrialDivision {
    fn prime_factorization(n: &U512) -> Vec<U512> {
        if n <= &U512::ONE {
            return vec![*n];
        }
        trial_div(*n)
    }
}

fn trial_div(mut n: U512) -> Vec<U512> {
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

fn is_still_undivided(n: &U512) -> bool {
    !n.is_one()
}

struct DivisorCandidates {
    current: U512,
}

impl DivisorCandidates {
    fn new() -> Self {
        DivisorCandidates { current: U512::TWO }
    }
}

impl Iterator for DivisorCandidates {
    type Item = U512;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.current;
        self.current = if self.current == U512::TWO {
            self.current + U512::ONE
        } else {
            self.current + U512::TWO
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
