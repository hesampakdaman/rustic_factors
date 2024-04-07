#![allow(dead_code)]

use crate::traits::{Factorize, PrimalityTest, PrimeFactorization};
use bnum::types::U512;

pub(crate) fn check_factorization<F: PrimeFactorization>(n: u32, factors: &[u32]) {
    let expected: Vec<U512> = factors.iter().map(|&d| U512::from(d)).collect();
    let mut actual = F::prime_factorization(&U512::from(n));
    actual.sort_unstable();
    assert_eq!(actual, expected, "Test failed for n = {}", n);
}

pub(crate) fn check_factor<F: Factorize>(n: u32, factor: u32) {
    let expected = U512::from(factor);
    let actual = F::factorize(&U512::from(n));
    assert_eq!(actual, expected, "Test failed for n = {}", n);
}

pub(crate) fn check_prime<P: PrimalityTest>(p: u32, expected: bool) {
    assert_eq!(P::is_prime(&U512::from(p)), expected);
}
