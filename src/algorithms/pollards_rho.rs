use rand::Rng;
mod utils;
use std::collections::VecDeque;

use crate::primality_test::MillerRabin;
use crate::traits::PrimalityTest;
use crate::Factorize;

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(&self, n: u128) -> Vec<u128> {
        if n <= 3 || MillerRabin.is_prime(n) {
            return vec![n];
        }
        let mut factors = vec![];
        let mut queue = VecDeque::from([n]);
        while let Some(m) = queue.pop_front() {
            if MillerRabin.is_prime(m) {
                factors.push(m);
                continue;
            }
            match pollards_rho(m) {
                FoundFactor::Prime(x) => {
                    factors.push(x);
                    queue.push_front(m / x)
                }
                FoundFactor::Composite(x) => {
                    queue.push_front(x);
                    queue.push_front(m / x)
                }
                FoundFactor::Trivial(_) => {
                    continue;
                }
            }
        }
        factors.sort_unstable();
        factors
    }
}

fn pollards_rho(n: u128) -> FoundFactor {
    let init = 2;
    let c = rand::thread_rng().gen_range(1..n);
    let finished = |x: u128, y: u128| utils::gcd(x.abs_diff(y), n) != 1;
    let psuedorandom = |x: u128| (x * x + c) % n;
    let (tortoise, hare) = utils::floyds_cycle_detection(init, &psuedorandom, &finished);
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
    fn simple() {
        assert_eq!(PollardsRho.factorize(8051), vec![83, 97]);
    }
}
