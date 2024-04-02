mod utils;

use crate::primality_test::MillerRabin;
use crate::traits::PrimalityTest;
use crate::Factorize;

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(&self, n: u128) -> Vec<u128> {
        RecursivePollardsRho::new(n).solve().factors
    }
}

struct RecursivePollardsRho {
    n: u128,
    factors: Vec<u128>,
}

impl RecursivePollardsRho {
    fn new(mut n: u128) -> Self {
        let mut factors = vec![];
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }
        Self { n, factors }
    }

    fn solve(mut self) -> Self {
        self.iter_pollars_rho(self.n);
        self
    }

    fn iter_pollars_rho(&mut self, n: u128) {
        if n <= 1 {
            return;
        }
        match pollards_rho(n) {
            DivisorOfN::Trivial(_) => self.iter_pollars_rho(n),
            DivisorOfN::Prime(p) => {
                self.factors.push(p);
                self.iter_pollars_rho(n / p)
            }
            DivisorOfN::Composite(d) => {
                self.iter_pollars_rho(n / d);
                self.iter_pollars_rho(d);
            }
        }
    }
}

fn pollards_rho(n: u128) -> DivisorOfN {
    let init = 2;
    let psudorandom_fn = utils::generate_psudeorandom_fn(n);
    let finished = |x: u128, y: u128| utils::gcd(x.abs_diff(y), n) != 1;
    let (tortoise, hare) = utils::floyds_cycle_detection(init, &psudorandom_fn, &finished);
    let d = utils::gcd(tortoise.abs_diff(hare), n);
    if MillerRabin.is_prime(d) {
        return DivisorOfN::Prime(d);
    }
    if d == 1 || d == n {
        return DivisorOfN::Trivial(d);
    }
    DivisorOfN::Composite(d)
}

enum DivisorOfN {
    Prime(u128),
    Composite(u128),
    Trivial(u128),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composites() {
        let test_cases = [(8051, vec![83, 97]), (15, vec![3, 5]), (4096, vec![2; 12])];
        for (n, expected) in test_cases {
            let mut actual = PollardsRho.factorize(n);
            actual.sort_unstable();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn primes() {
        let primes = [3, 5, 19, 29, 37, 7027, 13037];
        for p in primes {
            let actual = PollardsRho.factorize(p);
            assert_eq!(actual, vec![p]);
        }
    }
}
