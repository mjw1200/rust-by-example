fn finding1() {
    let mut quotient = 1f64;

    for i in 1..7
    {
        quotient /= 10f64;
        println!("{} {:.20}", i, quotient);
    }
}

fn finding2() {
    let quotient = 1f64;

    for i in 1..64
    {
        let denom = 10u128.pow(i);        
        let q64 = quotient / denom as f64;
        
        println!("{} {} {}", i, denom, q64);
    }
}

fn main() {
    finding1();
    // finding2();
}

// Findings:
// 1. There are 323 decimal places (I think) available with f64
// 2. You get less floating-point trash using / than /=
//   2a. With /= the fp trash starts at the 6th iteration
//   2b. With / the fp trash doesn't start until the 23rd iteration
//   2c. But... fp precision is hardware, not software. See main.c
//   2d. I'm not sure why the denominator calculation is overflowing...
