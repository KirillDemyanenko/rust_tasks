/*
    Напишите программу, которая считывает два целых числа A и B, а затем выводит наибольшее целое
    число X, для которого истинно высказывание:

    НЕ (X <= A) И (X < B)

    Если A и B не пересекаются, вывести A и B не пересекаются. Если пересекаются, но нет таких
    целых чисел, вывести Решения в целых числах нет.
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
    let a: i16 = input::<i16>();
    let b: i16 = input::<i16>();
    if a > b {
        println!("A и B не пересекаются");
    } else if a == b { 
        println!("Решения в целых числах нет")
    }
    for i in (a..=b).rev() {
        if !(i <= a) & (i < b) {
            println!("{i}");
            break;
        }
    }
}
