/*
    Дан знакочередующийся ряд натуральных чисел: S = 1 – 2 + 3 – 4 + 5 – 6 + 7 – 8 + ... ± n.

    Напишите программу, которая считывает натуральное число n и выводит сумму данного ряда
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
    let mut  s: i32 = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            s -= i
        } else { 
            s += i
        }
    }
    println!("{s}")
}