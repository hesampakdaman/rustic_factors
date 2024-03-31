use crate::Factorize;

pub struct TrialDivision;

impl Factorize for TrialDivision {
    fn factorize(&self, n: u128) -> Vec<u128> {
        if n <= 1 {
            return vec![n];
        }
        trial_div(n)
    }
}

fn trial_div(mut n: u128) -> Vec<u128> {
    let mut factors = vec![];
    let mut div = DivisorCandidates::new();
    while div.square() <= n {
        while div.divides(n) {
            factors.push(div.value());
            n /= div.value();
        }
        div.next();
    }
    if is_still_undivided(n) {
        factors.push(n);
    }
    factors
}

fn is_still_undivided(n: u128) -> bool {
    n > 1
}

struct DivisorCandidates(u128);

impl DivisorCandidates {
    fn new() -> Self {
        DivisorCandidates(2)
    }

    fn next(&mut self) -> &mut Self {
        self.0 += if self.0 == 2 { 1 } else { 2 };
        self
    }

    fn value(&self) -> u128 {
        self.0
    }

    fn square(&self) -> u128 {
        self.0 * self.0
    }

    fn divides(&self, n: u128) -> bool {
        n % self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorize_prime() {
        assert_eq!(TrialDivision.factorize(13), vec![13]);
    }

    #[test]
    fn factorize_composite() {
        assert_eq!(TrialDivision.factorize(12), vec![2, 2, 3]);
    }
}
