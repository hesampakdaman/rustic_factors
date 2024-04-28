use bnum::types::U512;
use rand::Rng;

pub fn floyds_cycle_detection<F, P>(init: U512, next: &F, finished: &P) -> (U512, U512)
where
    F: Fn(&U512) -> U512 + ?Sized,
    P: Fn(&U512, &U512) -> bool + ?Sized,
{
    let mut tortoise = init;
    let mut hare = next(&tortoise);
    while !finished(&tortoise, &hare) {
        tortoise = next(&tortoise);
        hare = next(&next(&hare));
    }
    (tortoise, hare)
}

pub fn generate_pseudorandom_fn(n: &'_ U512) -> impl Fn(&U512) -> U512 + '_ {
    let c = random_integer(n);
    move |x| (x.pow(2) + c) % n
}

fn random_integer(bound: &U512) -> U512 {
    rand::thread_rng().gen_range(U512::TWO..*bound)
}
