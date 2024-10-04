/*
    Напишите программу, которая считывает целое число num, натуральное значение
    n (u8) и столько же вводимых пользователем чисел, а затем выводит количество
    элементов больших num для считанной последовательности в виде сообщения:
    Количество элементов больших {num}: {}
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
    let num = input::<i32>();
    let n = input::<u8>();
    let mut result = 0;
    for _ in 0..n {
        if num < input::<i32>() {
            result += 1
        }
    }
    println!("Количество элементов больших {num}: {result}")
}
