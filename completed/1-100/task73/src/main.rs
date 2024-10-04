/*
    Напишите программу, которая считывает целое число, а затем выводит его цифры через пробел и
    значность с новой строк
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
    let mut num: i32 = input::<i32>();
    let tmp = num;
    let mut i: i32 = 0;
    let mut str = String::from("");
    while num > 0 {
        i += 1;
        str.push((num % 10).to_string().parse().unwrap());
        str.push(' ');
        num = num / 10;
    }
    println!("Число {} состоит из цифр: {}", tmp, str.chars().rev().collect::<String>().trim());
    println!("Число {} является {} значным", tmp, i)
}
