use crate::traits::PrimalityTest;
use bnum::types::U512;

pub(crate) fn check_prime<P: PrimalityTest>(p: u32, expected: bool) {
    assert_eq!(
        P::is_prime(&U512::from(p)),
        expected,
        "Test failed for prime = {}",
        p
    );
}
