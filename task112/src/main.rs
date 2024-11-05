/*
    Напишите программу, которая считывает натуральное число n (u16) и выводит первые n строк
    треугольника Паскаля.
*/
use std::fmt::Debug;
use std::str::FromStr;

pub fn input<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    buffer.trim().parse::<T>().expect("Parse error")
}

fn factorial(n: u32) -> u32 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn pascal_triangle(n: u32) -> Vec<u32> {
    let mut line: Vec<u32> = Vec::new();
    for i in 0..n + 1 {
        if i == 0 || i == n {
            line.push(1);
        } else if i == 1 || i == n - 1 {
            line.push(n as u32);
        } else if i == 2 || i == n - 2 {
            line.push((n * (n - 1)) / 2);
        } else if i == 3 || i == n - 3 {
            line.push(((n - 2) * (n - 1) * n) / 6)
        } else {
            line.push(factorial(n.into()) / (factorial(i) * (factorial(n - i))));
        }
    }
    line
}

fn main() {
    let n: u32 = input::<u32>() - 1;
    let width = pascal_triangle(n)
        .iter()
        .map(|el| el.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .len();
    for i in 0..=n {
        let row = pascal_triangle(i.into())
            .iter()
            .map(|el| el.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        print!(
            "{}{}\n",
            " ".repeat(((width - row.len()) as f64 / 2.0).ceil() as usize),
            row.trim()
        );
    }
}
