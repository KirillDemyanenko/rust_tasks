/*
    https://www.codewars.com/kata/57eaeb9578748ff92a000009/train/rust
*/

use either::Either;

fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    let mut sum = 0;
    for item in arr {
        match item { 
            Either::Left(num) => sum += num,
            Either::Right(s) => sum += s.parse::<i32>().unwrap(),
        }
    }
    sum
}

fn main() {
    println!(
        "{}",
        sum_mix(&[
            Either::Left(9),
            Either::Left(3),
            Either::Right("7".to_string()),
            Either::Right("3".to_string())
        ])
    );
    println!(
        "{}",
        sum_mix(&[
            Either::Left(8),
            Either::Left(0),
            Either::Left(0),
            Either::Left(8),
            Either::Left(5),
            Either::Left(7),
            Either::Left(2),
            Either::Left(3),
            Either::Left(7),
            Either::Left(8),
            Either::Left(6),
            Either::Left(7)
        ])
    );
}
