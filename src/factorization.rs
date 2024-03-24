use std::collections::BTreeMap;
use std::fmt;

use crate::Factorize;

static SUPERSCRIPTS: [&str; 10] = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];

pub struct Factorization {
    number: u128,
    factors: Vec<u128>,
}

impl Factorization {
    pub fn new(n: u128, f: impl Factorize) -> Self {
        Factorization {
            number: n,
            factors: f.factorize(n),
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

    fn frequencies(&self) -> BTreeMap<u128, usize> {
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

fn format_factor(base: u128, exp: usize) -> String {
    fn format_exp(exp: usize) -> String {
        if exp <= 1 {
            return "".to_string();
        }
        if exp <= 9 {
            return SUPERSCRIPTS[exp].to_string();
        }
        format!("^{}", exp)
    }
    format!("{}{}", base, format_exp(exp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn superscript() {
        let number = 36;
        let factors = vec![2, 2, 3, 3];
        let actual = Factorization { number, factors }.display();
        assert_eq!(actual, "36 = 2² x 3²");
    }

    #[test]
    fn fallback() {
        let number = 1024;
        let factors = vec![2; 10];
        let actual = Factorization { number, factors }.display();
        assert_eq!(actual, "1024 = 2^10");
    }
}
