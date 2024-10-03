/*
    Напишите программу, которая считывает два натуральных (u32) числа d и n, и n вводимых
    пользователем целых значений, а затем выводит среди них те, для которых d является делителем.
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
    let d = input::<u32>();
    let n = input::<u32>();
    for _ in 0..n {
        let num = input::<i32>();
        if num % d as i32 == 0 {
            println!("{num}");
        }
    }
}