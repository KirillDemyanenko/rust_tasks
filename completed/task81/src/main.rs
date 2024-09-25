/*
    Напишите программу, которая считывает показатели степени (u32) чисел 9 и 4, и выводит последнюю
    цифру суммы 9x + 4y без возведения
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
    let nine_degree: i64 = input::<i64>();
    let four_degree: i64 = input::<i64>();
    println!("Последняя цифра суммы равна {}",
             (if nine_degree % 2 == 0 { 1 } else { 9 } + if four_degree % 2 == 0 { 6 } else { 4 }) % 10);
}