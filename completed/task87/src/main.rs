/*
    Напишите программу, которая считывает два целых числа x (u32) и n (u8), считает произведение
x
∗
2
n
x∗2 
n
 , а затем выводит результат
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
    let x: u32 = input::<u32>();
    let n: u8 = input::<u8>();
    println!("{}", x << n)
}