/*
   Напишите программу, которая считывает вещественное число и выводит для него ближайшее целое 
   число (i64).

   Функцию round() реализовать вручную!
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
    println!("{}", round(input::<f64>()));
}

fn round(n: f64) -> i32 {
    if n == 0.0 {
        return 0;
    } else if n < 0.0 {
        return if n % 1.0 > -0.5 {
            (n - n % 1.0) as i32
        } else {
            (n - 1.0 - n % 1.0) as i32
        }
    } else {
        return if n % 1.0 < 0.5 {
            (n - n % 1.0) as i32
        } else {
            (n + 1.0 - n % 1.0) as i32
        }
    }
}
