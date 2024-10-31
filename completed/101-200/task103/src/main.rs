/*
    Напишите программу, которая считывает целые числа до ввода нуля и выводит сумму отрицательных и
    положительных значений.
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
    let mut n: i32 = input::<i32>();
    let mut total = (0, 0);
    while n != 0 {
        if n > 0 {total.0 += n} else {total.1 += n}
        n = input::<i32>()
    }
    println!("Сумма отрицательных чисел: {}", total.1);
    println!("Сумма положительных чисел: {}", total.0);
}