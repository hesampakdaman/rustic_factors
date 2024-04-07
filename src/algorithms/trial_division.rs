use crate::PrimeFactorization;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;

pub struct TrialDivision;

impl PrimeFactorization for TrialDivision {
    fn prime_factorization(n: &BigInt) -> Vec<BigInt> {
        if n <= &BigInt::one() {
            return vec![n.clone()];
        }
        trial_div(n.clone())
    }
}

fn trial_div(mut n: BigInt) -> Vec<BigInt> {
    let mut factors = vec![];
    let mut divisors = DivisorCandidates::new();
    while let Some(d) = divisors.next() {
        if n < d.pow(2) {
            break;
        }
        while n.is_multiple_of(&d) {
            n /= &d;
            factors.push(d.clone());
        }
    }
    if is_still_undivided(&n) {
        factors.push(n);
    }
    factors
}

fn is_still_undivided(n: &BigInt) -> bool {
    !n.is_one()
}

struct DivisorCandidates {
    current: BigInt,
}

impl DivisorCandidates {
    fn new() -> Self {
        DivisorCandidates {
            current: BigInt::from(2u8),
        }
    }
}

impl Iterator for DivisorCandidates {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.current.clone();
        self.current = if self.current == BigInt::from(2u8) {
            &self.current + 1u8
        } else {
            &self.current + 2u8
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
