use bnum::types::U256;
use std::collections::BTreeMap;
use std::fmt;

static SUPERSCRIPTS: [&str; 10] = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];

pub struct Factorization<'a> {
    number: &'a U256,
    factors: Vec<U256>,
}

impl<'a> Factorization<'a> {
    pub fn new(number: &'a U256, factors: Vec<U256>) -> Self {
        Self { number, factors }
    }

    pub fn display(&self) -> String {
        format!(
            "{} = {}",
            self.number,
            self.factor_frequencies()
                .iter()
                .map(|(&base, &exp)| format_factor(base, exp))
                .collect::<Vec<_>>()
                .join(" x ")
        )
    }

    fn factor_frequencies(&self) -> BTreeMap<&U256, u128> {
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

fn format_factor(base: &U256, exponent: u128) -> String {
    fn to_superscript(exp: u128) -> String {
        if exp <= 1 {
            return "".to_string();
        }
        exp.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .fold(String::new(), |s, idx| {
                format!("{}{}", s, SUPERSCRIPTS[idx])
            })
    }
    format!("{}{}", base, to_superscript(exponent))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: u32, factors: &[u32], expected: &str) {
        let n = U256::from(n);
        let factors = factors.iter().map(|&d| U256::from(d)).collect();
        let actual = Factorization::new(&n, factors);
        assert_eq!(format!("{actual}"), expected);
    }

    #[test]
    fn small_composite() {
        check(36, &[2, 2, 3, 3], "36 = 2² x 3²");
    }

    #[test]
    fn big_composite() {
        check(4096, &[2; 12], "4096 = 2¹²");
    }
}
