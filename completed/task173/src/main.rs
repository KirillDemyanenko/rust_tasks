/*
    https://www.codewars.com/kata/5287e858c6b5a9678200083c/train/rust
 */
use std::time::Instant;

fn narcissistic(num: u64) -> bool {
    let mut res = num;
    let mut numbers = vec![];
    while res > 0 {
        numbers.push(res % 10);
        res /= 10;
    }
    num == numbers.iter().map(|n| n.pow(numbers.len() as u32)).sum::<u64>()
}


fn main() {
    let start = Instant::now();
    for i in 1..709_551_615 {
        if narcissistic(i) {
            println!("{}", i);
        }
    }
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
