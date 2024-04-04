pub trait PrimeFactorization {
    fn prime_factorization(n: u128) -> Vec<u128>;
}

pub trait PrimalityTest {
    fn is_prime(p: u128) -> bool;
}
