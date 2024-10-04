/*
    Напишите программу, которая считывает количество элементов последовательности n (u8) и выводит
    последовательность Фибоначчи для числа n, как показано в примерах.
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
    let n: u8 = input::<u8>();
    let (mut n1, mut n2) = (0, 1);
    println!("Последовательность Фибоначчи для {n} элементов:");
    for i in 0..n {
        if i == 0 { println!("{n1}")}
        else if i == 1 { println!("{n2}") }
        else {
            (n1, n2) = (n2, n1 + n2);
            println!("{n2}");
        }
    }
}