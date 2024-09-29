/*
    Напишите программу, которая считывает натуральное число n (u8) и столько же вводимых
    пользователем целых чисел, а затем выводит результат сравнения для каждого числа с предыдущим,
    начиная со второго вводимого значения следующим образом:

    Если текущее число больше предыдущего, вывести: {текущее_число} > {предыдущее_число};
    Если текущее число меньше предыдущего, вывести:{текущее_число} < {предыдущее_число};
    Если текущее число и предыдущее равны, вывести:{текущее_число} == {предыдущее_число}.
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
    let n: i8 = input::<i8>();
    let mut prev: i8 = input::<i8>();
    let mut curr: i8;
    for _ in 1..n {
        curr = input::<i8>();
        println!("{curr} {} {prev}", if curr > prev {">"} else if curr < prev {"<"} else { "==" });
        prev = curr;
    }
}