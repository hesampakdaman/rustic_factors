use crate::Factorize;

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
        let factors = self
            .factors
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(" x ");
        format!("{} = {}", self.number, factors)
    }
}
