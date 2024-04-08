use bnum::types::U512;
use std::collections::BTreeMap;
use std::fmt;

use crate::PrimeFactorization;

static SUPERSCRIPTS: [&str; 10] = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];

pub struct Factorization<'a> {
    number: &'a U512,
    factors: Vec<U512>,
}

impl<'a> Factorization<'a> {
    pub fn new<F: PrimeFactorization>(n: &'a U512) -> Self {
        Factorization {
            number: n,
            factors: F::prime_factorization(n),
        }
    }

    pub fn display(&self) -> String {
        let display = self
            .frequencies()
            .iter()
            .map(|(&base, &exp)| format_factor(base, exp))
            .collect::<Vec<_>>()
            .join(" x ");
        format!("{} = {}", self.number, display)
    }

    fn frequencies(&self) -> BTreeMap<&U512, u128> {
        self.factors.iter().fold(BTreeMap::new(), |mut bmap, n| {
            *bmap.entry(n).or_insert(0) += 1;
            bmap
        })
    }
}

impl fmt::Display for Factorization<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

fn format_factor(base: &U512, exp: u128) -> String {
    fn format_exp(exp: u128) -> String {
        if exp <= 1 {
            return "".to_string();
        }
        exp.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .fold(String::new(), |s, d| format!("{}{}", s, SUPERSCRIPTS[d]))
    }
    format!("{}{}", base, format_exp(exp))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakePrimeFactorizer;

    impl PrimeFactorization for FakePrimeFactorizer {
        fn prime_factorization(n: &U512) -> Vec<U512> {
            if n == &U512::from(36u8) {
                return vec![2u8, 2, 3, 3].into_iter().map(U512::from).collect();
            } else {
                return vec![2u8; 12].into_iter().map(U512::from).collect();
            }
        }
    }

    fn check(n: u32, expected: &str) {
        let n = U512::from(n);
        let actual = Factorization::new::<FakePrimeFactorizer>(&n);
        assert_eq!(format!("{actual}"), expected);
    }

    #[test]
    fn small_composite() {
        check(36, "36 = 2² x 3²");
    }

    #[test]
    fn big_composite() {
        check(4096, "4096 = 2¹²");
    }
}
