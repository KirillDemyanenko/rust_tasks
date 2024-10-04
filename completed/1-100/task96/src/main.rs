/*
    Напишите программу, которая считывает натуральное число (u32) и выводит, является ли оно
    палиндром.

    Если является, вывести в виде сообщения: Число {} является палиндромом;
    Иначе: Число {} не является палиндромом.
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
    let n = input::<u32>();
    if n.to_string().chars().collect::<String>() == n.to_string().chars().rev().collect::<String>() {
        println!("Число {n} является палиндромом");
    } else {
        println!("Число {n} не является палиндромом");
    }
}