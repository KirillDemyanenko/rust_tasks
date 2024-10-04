/*
    Напишите программу, которая считывает три вещественных числа (x, a и b) и выводит (до 1 знака),
    принадлежит ли значение x интервалу (a, b).

    Если принадлежит вывести: Значение {x} принадлежит интервалу ({a}, {b}).
    Иначе вывести: Значение {x} не принадлежит интервалу ({a}, {b})
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
    let mut tup: (f64, f64, f64)  = (input::<f64>(), input::<f64>(), input::<f64>());
    if tup.1 > tup.2 {
        tup = (tup.0, tup.2, tup.1)
    }
    if tup.0 > tup.1 && tup.0 < tup.2 {
        println!("Значение {:.1} принадлежит интервалу ({:.1}, {:.1})", tup.0, tup.1, tup.2);
    } else {
        println!("Значение {:.1} не принадлежит интервалу ({:.1}, {:.1})", tup.0, tup.1, tup.2);
    }
}
