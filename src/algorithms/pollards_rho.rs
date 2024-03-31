mod utils;
use crate::Factorize;

pub struct PollardsRho;

impl Factorize for PollardsRho {
    fn factorize(&self, mut n: u128) -> Vec<u128> {
        if n <= 3 {
            return vec![n];
        }
        let mut factors = vec![];
        while let Ok(divisor) = pollards_rho(n) {
            while n % divisor == 0 {
                factors.push(divisor);
                n /= divisor;
            }
        }
        if n > 1 {
            factors.push(n);
        }
        factors.sort_unstable();
        factors
    }
}

pub fn pollards_rho(n: u128) -> Result<u128, &'static str> {
    let init = 2;
    let pseudorandom_sequence = |x| (x * x + 1) % n;
    let finished = |x: u128, y: u128| utils::gcd(x.abs_diff(y), n) != 1;
    let (tortoise, hare) = utils::floyds_cycle_detection(init, pseudorandom_sequence, finished);
    let divisor = utils::gcd(tortoise.abs_diff(hare), n);
    if n == divisor {
        return Err("Failed to find non-trivial factor");
    }
    Ok(divisor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(PollardsRho.factorize(8051), vec![83, 97]);
    }
}
