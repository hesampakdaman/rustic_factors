use crate::PrimeFactorization;
use bnum::types::U512;
use num_integer::Integer;
use num_traits::One;

pub struct TrialDivision;

impl PrimeFactorization for TrialDivision {
    fn prime_factorization(n: &U512) -> Vec<U512> {
        if n <= &U512::one() {
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
        DivisorCandidates {
            current: U512::from(2u8),
        }
    }
}

impl Iterator for DivisorCandidates {
    type Item = U512;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.current;
        self.current = if self.current == U512::from(2u8) {
            self.current + U512::from(1u8)
        } else {
            self.current + U512::from(2u8)
        };
        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utils::check_factorization;

    fn check(n: u32, factors: &[u32]) {
        check_factorization::<TrialDivision>(n, factors);
    }

    #[test]
    fn factorize_prime() {
        check(13, &[13]);
    }

    #[test]
    fn factorize_composite() {
        check(12, &[2, 2, 3]);
    }
}
