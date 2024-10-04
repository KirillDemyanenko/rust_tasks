/*
    Напишите программу, которая считывает два вещественных числа, представляющие собой координатную
    точку (x, y) и выводит, в какой четверти координатной плоскости находится точка с введенными
    координатами (x, y).

    Гарантируется, что точки не лежат на координатных осях.
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
    let x: f64 = input::<f64>();
    let y: f64 = input::<f64>();
    if x > 0.0 {
        if y > 0.0 {
            println!("I - четверть");
        } else {
            println!("IV - четверть");
        }
    } else {
        if y > 0.05 {
            println!("II - четверть");
        } else {
            println!("III - четверть");
        }
    }
}
