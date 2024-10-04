/*
    Напишите программу, которая считывает натуральное число n (usize) и выводит
    таблицу истинности для операции побитового И размерностью n.
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
    let n: usize = input::<usize>();
    let mut to = 0;
    for i in 0..n {
        to += u32::pow(2, i as u32);
    }
    for i in 0..=to {
        let s_str: String = format!("{i:0width$b}", width = n);
        println!("{} | {}", s_str, if s_str.contains("0") { 0 } else { 1 });
    }
}
