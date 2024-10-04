/*
    Дан факториал n! = 1 * 2 * 3 ... * n. Напишите программу, которая считывает натуральное число
    n (u32) и выводит последнюю цифру факториала
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
    let n: i32 = input::<i32>();
    match n { 
        1 => println!("Для {n}! последняя цифра равна 1"),
        2 => println!("Для {n}! последняя цифра равна 2"),
        3 => println!("Для {n}! последняя цифра равна 6"),
        4 => println!("Для {n}! последняя цифра равна 4"),
        _ => println!("Для {n}! последняя цифра равна 0")
    }
}