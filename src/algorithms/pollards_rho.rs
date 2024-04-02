use rand::Rng;
mod utils;
use std::ops::Range;

use crate::primality_test::MillerRabin;
use crate::traits::PrimalityTest;
use crate::Factorize;

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(&self, mut n: u128) -> Vec<u128> {
        if n <= 1 || MillerRabin.is_prime(n) {
            return vec![n];
        }
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
    if MillerRabin.is_prime(n) {
        factors.push(n);
        return;
    }
    let divisior_of_n = pollards_rho(n);
    iter_pollards_rho(n / divisior_of_n, factors);
    iter_pollards_rho(divisior_of_n, factors);
}

fn pollards_rho(n: u128) -> u128 {
    let init = 2;
    let psudorandom_fn = generate_psudeorandom_fn(n);
    let finished = |x: u128, y: u128| utils::gcd(x.abs_diff(y), n) != 1;
    let (tortoise, hare) = utils::floyds_cycle_detection(init, &psudorandom_fn, &finished);
    utils::gcd(tortoise.abs_diff(hare), n)
}

fn generate_psudeorandom_fn(n: u128) -> impl Fn(u128) -> u128 {
    let c = random_integer(1..n);
    move |x| (x * x + c) % n
}

fn random_integer(r: Range<u128>) -> u128 {
    rand::thread_rng().gen_range(r)
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
