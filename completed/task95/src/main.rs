/*
    Напишите программу, которая считывает целое число (u8) и выводит количество установленных битов
    в его двоичной записи в виде сообщения:Количество установленных битов в {:08b} равно {}.
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
    let x:u8 = input::<u8>();
    let mut count: u8 = 0;
    let mut i = 0;
    while i < 8 {
        count += if x & (1 << i) != 0 {1} else {0};
        i += 1;
    }
    println!("Количество установленных битов в {:08b} равно {}", x, count);
}