/*
    Напишите программу, которая считывает два целых числа, меняет их значения местами без
    использования третьей переменной с помощью операции XOR, а затем выводит получившиеся переменные.

    Так, если переменные a = 5, b = 10, то после обмена значениями должно стать a = 10, а b = 5 без
    использования дополнительной переменной.
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
    let mut a: i16 = input::<i16>();
    let mut b: i16 = input::<i16>();
    a ^= b;
    b ^= a;
    a ^= b;
    println!("{a}");
    println!("{b}");
}