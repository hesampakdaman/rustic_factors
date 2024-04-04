use std::collections::BTreeMap;
use std::fmt;

use crate::PrimeFactorization;

static SUPERSCRIPTS: [&str; 10] = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];

pub struct Factorization {
    number: u128,
    factors: Vec<u128>,
}

impl Factorization {
    pub fn new<F: PrimeFactorization>(n: u128) -> Self {
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

    fn frequencies(&self) -> BTreeMap<u128, u128> {
        self.factors.iter().fold(BTreeMap::new(), |mut bmap, &n| {
            *bmap.entry(n).or_insert(0) += 1;
            bmap
        })
    }
}

impl fmt::Display for Factorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

fn format_factor(base: u128, exp: u128) -> String {
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

    #[test]
    fn small_composite() {
        let number = 36;
        let factors = vec![2, 2, 3, 3];
        let actual = Factorization { number, factors };
        assert_eq!(format!("{actual}"), "36 = 2² x 3²");
    }

    #[test]
    fn big_composite() {
        let number = 2u128.pow(12);
        let factors = vec![2; 12];
        let actual = Factorization { number, factors };
        assert_eq!(format!("{actual}"), "4096 = 2¹²");
    }

}
