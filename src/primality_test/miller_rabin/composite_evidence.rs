use super::utils;
use num_bigint::BigUint;
use num_traits::One;

pub struct CompositeEvidence<'a> {
    n: &'a BigUint,
    n_minus_1: Decomposed,
}

impl<'a> CompositeEvidence<'a> {
    pub fn new(n: &'a BigUint) -> Self {
        let n_minus_1 = Decomposed::new(n - 1u8);
        Self { n, n_minus_1 }
    }

    pub fn witnessed_by(&self, witness: &BigUint) -> bool {
        match self.raise_to_n_minus_1_mod_n(witness) {
            Ok(result) => fails_fermats_condition(result),
            Err(FoundNonTrivialSqrtOf1) => true,
        }
    }

    fn raise_to_n_minus_1_mod_n(&self, base: &BigUint) -> ExponentiationResult {
        let odd_factor_in_exp = &self.n_minus_1.odd_factor;
        let mut result = base.modpow(odd_factor_in_exp, &self.n);
        for _ in 0..self.n_minus_1.exponent_of_2 {
            if self.is_nontrivial_sqrt_of_1(&result) {
                return Err(FoundNonTrivialSqrtOf1);
            }
            result = result.modpow(&BigUint::from(2u8), &self.n);
        }
        Ok(RaisedToNMinus1ModN(result))
    }

    pub fn is_nontrivial_sqrt_of_1(&self, solution: &BigUint) -> bool {
        let squared = solution.modpow(&BigUint::from(2u8), &self.n);
        squared == BigUint::one() && solution != &BigUint::one() && solution != &(self.n - 1u8)
    }
}

fn fails_fermats_condition(r: RaisedToNMinus1ModN) -> bool {
    !r.0.is_one()
}

type ExponentiationResult = Result<RaisedToNMinus1ModN, FoundNonTrivialSqrtOf1>;

struct RaisedToNMinus1ModN(BigUint);

struct FoundNonTrivialSqrtOf1;

struct Decomposed {
    exponent_of_2: u32,
    odd_factor: BigUint,
}

impl Decomposed {
    /// Decomposes `number` into `exponent_of_2` and `odd_factor`,
    /// where `number = 2^exponent_of_2 * odd_factor`.
    pub fn new(number: BigUint) -> Self {
        let exponent_of_2 = utils::highest_power_of_2_divisor(&number);
        let odd_factor = number / BigUint::from(2u8).pow(exponent_of_2);
        Self {
            exponent_of_2,
            odd_factor,
        }
    }
}
