use super::utils;

pub struct Witnessed {
    number: u128,
    n_minus_1_exp: Decomposed,
}

impl Witnessed {
    pub fn new(number: u128) -> Self {
        Self {
            number,
            n_minus_1_exp: Decomposed::new(number - 1),
        }
    }

    pub fn by(&self, base: u128) -> bool {
        match self.find_nontrivial_sqrt_of_1(base) {
            SqrtOfOne::NonTrivial(_) => true,
            SqrtOfOne::TrivialOnly(x) => !utils::passes_fermats_condition(x),
        }
    }

    fn find_nontrivial_sqrt_of_1(&self, base: u128) -> SqrtOfOne {
        let odd_factor_in_exp = self.n_minus_1_exp.odd_factor;
        let mut solution = utils::modular_exponentiation(base, odd_factor_in_exp, self.number);
        for _ in 0..self.n_minus_1_exp.exponent_of_2 {
            if utils::is_nontrivial_sqrt_of_1(solution, self.number) {
                return SqrtOfOne::NonTrivial(solution);
            }
            solution = (solution * solution) % self.number;
        }
        SqrtOfOne::TrivialOnly(solution)
    }
}

enum SqrtOfOne {
    NonTrivial(u128),
    TrivialOnly(u128),
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
