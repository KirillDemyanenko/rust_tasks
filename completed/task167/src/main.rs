/*
   Напишите программу, которая считывает радиус (f64) и выводит (до 3 знаков) площадь круга.
*/
use std::f64::consts::PI;
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
    println!("{:.3}", PI * input::<f64>().powi(2))
}
