use std::env;
use std::time::{Instant};

mod seive;

fn main() {

	let args: Vec<String> = env::args().collect();
    let max = &args[1];
    let parse_max = max.parse().unwrap();

    println!("All Primes till {}", parse_max);

    let start = Instant::now();
    let result = seive::get_largest_prime(parse_max);
    let duration = start.elapsed();

    println!("");
    println!("{:?}", result);
    println!("Total time elasped : {:?}", duration);
}
