/*
    Напишите программу, которая считывает показатель степени (целое
    положительное число) числа 2 и выводит его последнюю цифру без возведения
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
    let num: u32 = input::<u32>();
    if num % 4 == 0 {
        println!("Последняя цифра 2 в степени {num} равна 6")
    } else if num % 4 == 3 {
        println!("Последняя цифра 2 в степени {num} равна 8")
    } else if num % 4 == 2 {
        println!("Последняя цифра 2 в степени {num} равна 4")
    } else {
        println!("Последняя цифра 2 в степени {num} равна 2")
    }
}
