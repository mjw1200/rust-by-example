extern crate sort;

use rand::{Rng};
use sort::bubble_sort;
use sort::quicksort;
use chrono;

fn main() {
    const ELEMENTS:usize = 10000;
    const HIGH:u32 = 100;

    let mut data = [0u32; ELEMENTS];

    let mut rng = rand::thread_rng();

    for i in 1..ELEMENTS {
        data[i] = rng.gen_range(0..HIGH);
    }
    
    let start = chrono::offset::Utc::now();
    quicksort(&mut data);
    let stop = chrono::offset::Utc::now();

    println!("Sort took {}", stop - start);
}
