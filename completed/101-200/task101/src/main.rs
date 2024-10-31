/*
    Напишите программу, которая считывает вещественные числа и выводит среди них первый положительный элемент.
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
    let mut n: f64;
    loop {
        n = input::<f64>();
        if n > 0.0 { break }
    }
    println!("{n}")
}