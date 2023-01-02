use rand::{Rng};

fn main() {
    const ELEMENTS:usize = 1000;
    const HIGH:u32 = 100;

    let mut occurs = [0; HIGH as usize];

    let mut rng = rand::thread_rng();

    for _ in 1..ELEMENTS {
        let val = rng.gen_range(0..HIGH);
        occurs[val as usize] += 1;
    }

    for i in 0..HIGH {
        println!("{}: {}", i, occurs[i as usize]);
    }
}
