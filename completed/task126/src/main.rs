/*
    https://stepik.org/lesson/1300620/step/3?unit=1315402
*/
use std::fmt::Debug;
use std::str::FromStr;

fn input<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    buffer.trim().parse::<T>().expect("Parse error")
}

fn main() {
    let n: u32 = input::<u32>();
    let x: f64 = input::<f64>();
    let mut p: f64 = 1.0;
    for _ in 0..=n {
        p *= (x + 1.0) / (x.powi(2) / 2.0 + 4.0)
    }
    println!("{:.5}", p);
}
