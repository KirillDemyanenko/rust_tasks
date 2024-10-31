/*
    Напишите программу, которая считывает два целых числа и выводит их сумму, не используя
    какие-либо арифметические операторы (+, -, .. и т.д.), только побитовые.
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
    let mut a: i16 = input::<i16>();
    let mut b: i16 = input::<i16>();
    while b != 0 {
        let c = a;
        a = a ^ b;
        b = (c & b) << 1;
    }
    println!("{a}")
}
