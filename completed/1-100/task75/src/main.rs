/*
    Напишите программу, которая считывает три целых числа (x, a и b) и выводит, принадлежит ли
    значение x отрезку [a, b].

    Если принадлежит вывести: Значение {x} принадлежит отрезку [{a}, {b}].
    Иначе вывести: Значение {x} не принадлежит отрезку [{a}, {b}].
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
    return buffer.trim().parse::<T>().expect("Parse error");
}

fn main() {
    let mut tup: (i32, i32, i32)  = (input::<i32>(), input::<i32>(), input::<i32>());
    if tup.1 > tup.2 {
        tup = (tup.0, tup.2, tup.1)
    }
    if tup.0 >= tup.1 && tup.0 <= tup.2 {
        println!("Значение {} принадлежит отрезку [{}, {}]", tup.0, tup.1, tup.2);
    } else {
        println!("Значение {} не принадлежит отрезку [{}, {}]", tup.0, tup.1, tup.2);
    }
}
