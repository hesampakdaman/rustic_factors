mod utils;

use crate::primality_test::MillerRabin;
use crate::traits::PrimalityTest;
use crate::Factorize;

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(&self, mut n: u128) -> Vec<u128> {
        let mut factors = vec![];
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }
        iter_pollards_rho(n, &mut factors);
        factors
    }
}

fn iter_pollards_rho(n: u128, factors: &mut Vec<u128>) {
    if n <= 1 {
        return;
    }
    match pollards_rho(n) {
        DivisorOfN::Trivial(_) => iter_pollards_rho(n, factors),
        DivisorOfN::Prime(p) => {
            factors.push(p);
            iter_pollards_rho(n / p, factors)
        }
        DivisorOfN::Composite(d) => {
            iter_pollards_rho(n / d, factors);
            iter_pollards_rho(d, factors);
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
