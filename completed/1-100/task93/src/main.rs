/*
    Напишите программу, которая считывает два целых числа x (i8) и n (u8), переключает n-й бит
    переменной x и выводит получившееся число в двоичной и десятичной записи в виде сообщений
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
    let mut  x: i8 = input::<i8>();
    let n: u8 = input::<u8>();
    println!("x до переключения {n}-го бита");
    println!("в двоичной записи: {:08b}", x);
    println!("в десятичной записи: {x}\n");
    x = x ^ (1 << n);
    println!("x после переключения {n}-го бита");
    println!("в двоичной записи: {:08b}", x);
    println!("в десятичной записи: {x}");
}