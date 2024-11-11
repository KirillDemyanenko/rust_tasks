/*
   Напишите программу, которая считывает размер файла в байтах (u64) и время передачи файла (f64),
   а затем выводит (до 2 знаков) скорость передачи данных в Мбит/с
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
    let file_size = input::<u64>();
    let time = input::<f64>();
    println!(
        "Скорость передачи данных: {:.2} Мбит/с",
        (file_size as f64 * 8.0) / (time * 1_000_000.0)
    );
}
