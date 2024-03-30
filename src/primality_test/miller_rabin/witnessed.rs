use super::utils;

pub struct Witnessed {
    number: u128,
    n_minus_1: Decomposed,
}

impl Witnessed {
    pub fn new(number: u128) -> Self {
        let n_minus_1 = Decomposed::new(number - 1);
        Self { number, n_minus_1 }
    }

    pub fn not_by(&self, witness_candidate: u128) -> bool {
        match self.raise_to_n_minus_1(witness_candidate) {
            Ok(result) => passes_fermats_condition(result),
            Err(ExponentiationErr::FoundNonTrivialSqrtOf1) => false,
        }
    }

    fn raise_to_n_minus_1(&self, base: u128) -> Result<RaisedToNMinus1, ExponentiationErr> {
        let odd_factor_in_exp = self.n_minus_1.odd_factor;
        let mut result = utils::modular_exponentiation(base, odd_factor_in_exp, self.number);
        for _ in 0..self.n_minus_1.exponent_of_2 {
            if utils::is_nontrivial_sqrt_of_1(result, self.number) {
                return Err(ExponentiationErr::FoundNonTrivialSqrtOf1);
            }
            result = (result * result) % self.number;
        }
        Ok(RaisedToNMinus1(result))
    }
}

fn passes_fermats_condition(r: RaisedToNMinus1) -> bool {
    r.0 == 1
}

struct RaisedToNMinus1(u128);

enum ExponentiationErr {
    FoundNonTrivialSqrtOf1,
}

struct Decomposed {
    exponent_of_2: u32,
    odd_factor: u128,
}

impl Decomposed {
    /// Decomposes `number` into `exponent_of_2` and `odd_factor`,
    /// where `number = 2^exponent_of_2 * odd_factor`.
    pub fn new(number: u128) -> Self {
        let exponent_of_2 = utils::highest_power_of_2_divisor(number);
        let odd_factor = number / 2u128.pow(exponent_of_2);
        Self {
            exponent_of_2,
            odd_factor,
        }
    }
}
