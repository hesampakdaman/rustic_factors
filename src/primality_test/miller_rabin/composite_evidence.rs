use super::utils;
use bnum::types::U256;
use num_integer::Integer;
use num_traits::One;

pub struct CompositeEvidence<'a> {
    n: &'a U256,
    n_minus_1: Decomposed,
}

impl<'a> CompositeEvidence<'a> {
    pub fn new(n: &'a U256) -> Self {
        let n_minus_1 = Decomposed::new(n - U256::ONE);
        Self { n, n_minus_1 }
    }

    pub fn witnessed_by(&self, witness: &U256) -> bool {
        match self.raise_to_n_minus_1_mod_n(witness) {
            Ok(result) => fails_fermats_condition(result),
            Err(FoundNonTrivialSqrtOf1) => true,
        }
    }

    fn raise_to_n_minus_1_mod_n(&self, base: &U256) -> ExponentiationResult {
        let odd_factor_in_exp = &self.n_minus_1.odd_factor;
        let mut result = modpow(base, odd_factor_in_exp, self.n);
        for _ in 0..self.n_minus_1.exponent_of_2 {
            if self.is_nontrivial_sqrt_of_1(&result) {
                return Err(FoundNonTrivialSqrtOf1);
            }
            result = modpow(&result, &U256::TWO, self.n);
        }
        Ok(RaisedToNMinus1ModN(result))
    }

    pub fn is_nontrivial_sqrt_of_1(&self, solution: &U256) -> bool {
        let squared = modpow(solution, &U256::TWO, self.n);
        squared == U256::one()
            && solution != &U256::ONE
            && solution != &(self.n - U256::ONE)
    }
}

fn modpow(base: &U256, exponent: &U256, modulus: &U256) -> U256 {
    let mut result = U256::ONE;
    let mut base = base % modulus;
    let mut exp = *exponent;
    while exp > U256::ZERO {
        if exp.is_odd() {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= U256::TWO;
    }
    result
}

fn fails_fermats_condition(r: RaisedToNMinus1ModN) -> bool {
    !r.0.is_one()
}

type ExponentiationResult = Result<RaisedToNMinus1ModN, FoundNonTrivialSqrtOf1>;

struct RaisedToNMinus1ModN(U256);

struct FoundNonTrivialSqrtOf1;

struct Decomposed {
    exponent_of_2: u32,
    odd_factor: U256,
}

impl Decomposed {
    /// Decomposes `number` into `exponent_of_2` and `odd_factor`,
    /// where `number = 2^exponent_of_2 * odd_factor`.
    pub fn new(number: U256) -> Self {
        let exponent_of_2 = utils::highest_power_of_2_divisor(&number);
        let odd_factor = number / U256::TWO.pow(exponent_of_2);
        Self {
            exponent_of_2,
            odd_factor,
        }
    }
}
