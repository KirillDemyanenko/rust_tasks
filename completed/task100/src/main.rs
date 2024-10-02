/*
    Напишите программу, которая считывает два натуральных (u32) числа n и k соответственно, а затем
    выводит число, полученное удалением первых k цифр из числа n. Если количество удаляемых цифр
    нестрого больше, чем количество цифр в n, то выводится сообщение k >= n.
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
    let binding = input::<u32>().to_string();
    let n = if binding.len() > 1 {binding.split("").collect::<Vec<_>>()} else {Vec::new()};
    let k = input::<u32>();
    if k >= n.len() as u32
    { 
        println!("k >= n")
    } else { 
        println!("{}", n[(k as usize + 1)..n.len()].join("").parse::<u32>().unwrap());
    }
}