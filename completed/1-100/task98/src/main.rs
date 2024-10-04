/*
    Напишите программу, которая считывает натуральное число b (u16) и выводит значение log2(b)
    используя цикл и битовые операции.
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
    let n = input::<u16>();
    let mut count = 0;
    while 2 << count != n {
        count += 1;
    }
    println!("{}", count + 1);
}