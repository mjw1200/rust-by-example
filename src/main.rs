extern crate rand;
extern crate rand_chacha;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

fn main() {
    let mut rng = ChaCha20Rng::from_entropy();
    println!("{}", rng.gen_range(0..100));
}
