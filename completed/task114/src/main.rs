/*
    Напишите программу, которая считывает натуральное число (u8) и выводит, является ли оно простым
    или нет. Если является, вывести Число {} является простым, иначе Число {} не является простым.
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
    let mut is_simple = true;
    for i in 2..n - 1 {
        if n % i == 0 {
            is_simple = false;
            break
        }
    }
    println!("Число {n} {}является простым", if is_simple { "" } else { "не " });
}
