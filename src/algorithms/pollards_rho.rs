mod utils;

use std::marker::PhantomData;

use crate::primality_test;
use crate::traits::PrimalityTest;
use crate::PrimeFactorization;

pub struct PollardsRho;

impl PrimeFactorization for PollardsRho {
    fn prime_factorization(n: u128) -> Vec<u128> {
        RecursivePollardsRho::<primality_test::MillerRabin>::new(n)
            .solve()
            .factors
    }
}

struct RecursivePollardsRho<P: PrimalityTest> {
    n: u128,
    factors: Vec<u128>,
    prime_tester: PhantomData<P>,
}

impl<P: PrimalityTest> RecursivePollardsRho<P> {
    fn new(mut n: u128) -> Self {
        let mut factors = vec![];
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }
        Self {
            n,
            factors,
            prime_tester: PhantomData,
        }
    }

    fn solve(mut self) -> Self {
        self.recursively_factorize_n(self.n);
        self
    }

    fn recursively_factorize_n(&mut self, n: u128) {
        if n <= 1 {
            return;
        }
        match self.get_divisor_with_pollards_rho(n) {
            DivisorOfN::Trivial(_) => self.recursively_factorize_n(n),
            DivisorOfN::Prime(p) => {
                self.factors.push(p);
                self.recursively_factorize_n(n / p)
            }
            DivisorOfN::Composite(d) => {
                self.recursively_factorize_n(n / d);
                self.recursively_factorize_n(d);
            }
        }
    }

    fn get_divisor_with_pollards_rho(&self, n: u128) -> DivisorOfN {
        let d = pollards_rho(n);
        if P::is_prime(d) {
            return DivisorOfN::Prime(d);
        }
        if d == 1 || d == n {
            return DivisorOfN::Trivial(d);
        }
        DivisorOfN::Composite(d)
    }
}

fn pollards_rho(n: u128) -> u128 {
    let init = 2;
    let psudorandom_fn = utils::generate_psudeorandom_fn(n);
    let finished = |x: u128, y: u128| utils::gcd(x.abs_diff(y), n) != 1;
    let (tortoise, hare) = utils::floyds_cycle_detection(init, &psudorandom_fn, &finished);
    utils::gcd(tortoise.abs_diff(hare), n)
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
            let mut actual = PollardsRho::prime_factorization(n);
            actual.sort_unstable();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn primes() {
        let primes = [3, 5, 19, 29, 37, 7027, 13037];
        for p in primes {
            let actual = PollardsRho::prime_factorization(p);
            assert_eq!(actual, vec![p]);
        }
    }
}
