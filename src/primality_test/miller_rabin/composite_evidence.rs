use super::utils;

pub struct CompositeEvidence {
    n: u128,
    n_minus_1: Decomposed,
}

impl CompositeEvidence {
    pub fn new(n: u128) -> Self {
        let n_minus_1 = Decomposed::new(n - 1);
        Self { n, n_minus_1 }
    }

    pub fn witnessed_by(&self, witness: u128) -> bool {
        match self.raise_to_n_minus_1_mod_n(witness) {
            Ok(result) => fails_fermats_condition(result),
            Err(FoundNonTrivialSqrtOf1) => true,
        }
    }

    fn raise_to_n_minus_1_mod_n(&self, base: u128) -> ExponentiationResult {
        let odd_factor_in_exp = self.n_minus_1.odd_factor;
        let mut result = utils::modular_exponentiation(base, odd_factor_in_exp, self.n);
        for _ in 0..self.n_minus_1.exponent_of_2 {
            if utils::is_nontrivial_sqrt_of_1(result, self.n) {
                return Err(FoundNonTrivialSqrtOf1);
            }
            result = utils::modular_exponentiation(result, 2, self.n);
        }
        Ok(RaisedToNMinus1ModN(result))
    }
}

fn fails_fermats_condition(r: RaisedToNMinus1ModN) -> bool {
    r.0 != 1
}

type ExponentiationResult = Result<RaisedToNMinus1ModN, FoundNonTrivialSqrtOf1>;

struct RaisedToNMinus1ModN(u128);

struct FoundNonTrivialSqrtOf1;

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
