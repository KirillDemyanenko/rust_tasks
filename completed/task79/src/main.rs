/*
    Напишите программу, которая считывает три целых числа (a, b и c) и выводит, являются ли эти три
    числа длинами сторон треугольника.

    Если да, то вывести, какого треугольника (разносторонний, равнобедренный или равносторонний)
    следующим образом:
     - Числа {a}, {b} и {c} образуют разносторонний треугольник или
     - Числа {a}, {b} и {c} образуют равнобедренный треугольник или
     - Числа {a}, {b} и {c} образуют равносторонний треугольник.
     - Иначе вывести: Числа {a}, {b} и {c} не образуют треугольник.
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
    let x: (i32, i32, i32) = (input::<i32>(), input::<i32>(), input::<i32>());
    if x.0 + x.1 > x.2 && x.0 + x.2 > x.1 && x.1 + x.2 > x.0 {
        if x.0 == x.1 && x.1 == x.2 {
            println!("Числа {}, {} и {} образуют равносторонний треугольник", x.0, x.1, x.2);
        } else if x.0 != x.1 && x.1 != x.2 && x.2 != x.0 {
            println!("Числа {}, {} и {} образуют разносторонний треугольник", x.0, x.1, x.2);
        } else {
            println!("Числа {}, {} и {} образуют равнобедренный треугольник", x.0, x.1, x.2);
        }
    } else {
        println!("Числа {}, {} и {} не образуют треугольник", x.0, x.1, x.2);
    }
}
