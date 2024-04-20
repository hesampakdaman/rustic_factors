use bnum::types::U256;

pub trait Factorize {
    fn factorize(n: &U256) -> U256;
}

pub trait PrimeFactorization {
    fn prime_factorization(n: &U256) -> Vec<U256>;
}

pub trait PrimalityTest {
    fn is_prime(p: &U256) -> bool;
}

pub trait Command {
    fn run(&self, n: &U256) -> String;
}

pub trait FactorizationCommand: Command + PrimeFactorization {}
