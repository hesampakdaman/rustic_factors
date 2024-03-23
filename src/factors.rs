use crate::Factorize;

pub struct Factors {
    number: u128,
    result: Vec<u128>,
}

impl Factors {
    pub fn new(f: impl Factorize, n: u128) -> Self {
        Factors {
            number: n,
            result: f.factorize(n),
        }
    }

    pub fn display(&self) -> String {
        let factors = self
            .result
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(" x ");
        format!("{} = {}", self.number, factors)
    }
}
