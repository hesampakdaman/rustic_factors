use bnum::types::U512;

pub trait PrimeFactorization {
    fn prime_factorization(n: &U512) -> Vec<U512>;
}

pub trait Factorize {
    fn factorize(n: &U512) -> U512;
}

pub trait PrimalityTest {
    fn is_prime(p: &U512) -> bool;
}

pub trait Command {
    fn run(&self, n: &U512) -> String;
}

pub trait FactorizationCommand: Command + PrimeFactorization {}
