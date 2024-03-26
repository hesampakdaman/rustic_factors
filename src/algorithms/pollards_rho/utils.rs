pub fn floyds_cycle_detection<F, P>(init: u128, next: F, finished: P) -> (u128, u128)
where
    F: Fn(u128) -> u128,
    P: Fn(u128, u128) -> bool,
{
    let mut tortoise = init;
    let mut hare = next(tortoise);
    while !finished(tortoise, hare) {
        tortoise = next(tortoise);
        hare = next(next(hare));
    }
    (tortoise, hare)
}

pub fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
