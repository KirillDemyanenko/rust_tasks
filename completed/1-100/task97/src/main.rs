/*
    Напишите программу, которая считывает натуральное число (u32) и выводит, являются ли все его
    цифры одинаковыми.

    Если все цифры равны, вывести сообщение Все цифры числа {} равны;
    Иначе Цифры числа {} неодинаковые.
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
    let mut n = input::<u32>();
    let n_copy = n;
    let ch = n % 10;
    while n > 0 {
        if n % 10 != ch {
            println!("Цифры числа {n_copy} неодинаковые");
            break;
        }
        n /= 10;
    }
    if n == 0 {
        println!("Все цифры числа {n_copy} равны")
    }
}