/*
    Напишите программу, которая считывает два целых числа x (u8) и n (u8), проверяет на установку
    n-й бит переменной x и выводит получившееся число в двоичной записи в виде сообщения
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
    let x = input::<u8>();
    let n: u8 = input::<u8>();
    let bit = match format!("{:08b}", x).chars().rev().collect::<String>().chars().nth(n as usize) {
        Some(x) => x,
        _ => {'0'}
    };
    println!("{n}-й бит числа {:08b} равен {}", x, bit);
}