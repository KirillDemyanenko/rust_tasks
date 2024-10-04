/*
    Программа должна считать неотрицательное (u32) число n и вещественное
    число x, а затем вывести (до 3 знаков) корень n-й степени с точностью 1e-15
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
    return buffer.trim().parse::<T>().expect("Parse error");
}

fn main() {
    let n: u32 = input::<u32>();
    let x: f64 = input::<f64>();
    println!("{:.3}", x.powf(1.0 / n as f64))
}
