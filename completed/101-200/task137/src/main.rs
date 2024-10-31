/*
    Напишите программу, которая считывает два целых числа (u8) и выводит количество битов, которые
    необходимо переключить, чтобы преобразовать первое число во второе.
*/
use std::fmt::Debug;
use std::str::FromStr;

pub fn input<T: FromStr>() -> T
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
    let a = format!("{:08b}", input::<u8>());
    let b = format!("{:08b}", input::<u8>());
    let a_vec = a.split("").collect::<Vec<&str>>();
    let b_vec = b.split("").collect::<Vec<&str>>();
    let mut count = 0;
    for i in 1..=a.len() {
        if a_vec[i] != b_vec[i] {
            count += 1;
        }
    }
    println!("{}", count);
}
