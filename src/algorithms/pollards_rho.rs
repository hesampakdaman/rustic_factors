use rand::Rng;
mod utils;
use std::collections::VecDeque;
use std::ops::Range;

use crate::primality_test::MillerRabin;
use crate::traits::PrimalityTest;
use crate::Factorize;

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(&self, mut n: u128) -> Vec<u128> {
        if n <= 3 || MillerRabin.is_prime(n) {
            return vec![n];
        }
        let mut factors = vec![];
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }
        if n <= 1 {
            return factors;
        }
        let mut queue = VecDeque::from([n]);
        let mut f = generate_psudeorandom_fn(n);
        while let Some(m) = queue.pop_front() {
            if m <= 1 {
                continue;
            }
            if MillerRabin.is_prime(m) {
                factors.push(m);
                continue;
            }
            match pollards_rho(m, &f) {
                FoundFactor::Prime(x) => {
                    factors.push(x);
                    queue.push_front(m / x)
                }
                FoundFactor::Composite(x) => {
                    queue.push_front(x);
                    queue.push_front(m / x)
                }
                FoundFactor::Trivial(x) => {
                    queue.push_front(x);
                    f = generate_psudeorandom_fn(n);
                }
            }
        }
        factors
    }
}

fn generate_psudeorandom_fn(n: u128) -> impl Fn(u128) -> u128 {
    let c = random_integer(1..n);
    move |x| (x * x + c) % n
}

fn random_integer(r: Range<u128>) -> u128 {
    rand::thread_rng().gen_range(r)
}

fn pollards_rho(n: u128, psudorandom_fn: impl Fn(u128) -> u128) -> FoundFactor {
    let init = 2;
    let finished = |x: u128, y: u128| utils::gcd(x.abs_diff(y), n) != 1;
    let (tortoise, hare) = utils::floyds_cycle_detection(init, &psudorandom_fn, &finished);
    let divisor = utils::gcd(tortoise.abs_diff(hare), n);
    if divisor == 1 || divisor == n {
        FoundFactor::Trivial(divisor)
    } else if MillerRabin.is_prime(divisor) {
        FoundFactor::Prime(divisor)
    } else {
        FoundFactor::Composite(divisor)
    }
}

enum FoundFactor {
    Trivial(u128),
    Composite(u128),
    Prime(u128),
}

#[derive(Debug)]
struct FailedToFindNonTrivialFactor;

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
