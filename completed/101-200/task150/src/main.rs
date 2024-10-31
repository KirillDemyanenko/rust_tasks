/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считывать вещественное число и выводить его округления до ближайшего наибольшего и наименьшего
   целого числа.

    Округления до ближайшего наибольшего и наименьшего целого числа реализуйте в соответствующих
    функциях ceil() и floor()!
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
    let num: f64 = input::<f64>();
    println!("{}", ceil(num));
    println!("{}", floor(num));
}

fn ceil(num: f64) -> i64 {
    if num % 1.0 == 0.0 {
        return num as i64
    }
    if num > 0.0 {
        (num - num % 1.0 + 1.0) as i64
    } else {
        (num - num % 1.0) as i64
    }
}

fn floor(num: f64) -> i64 {
    if num % 1.0 == 0.0 {
        return num as i64
    }
    if num > 0.0 {
        (num - num % 1.0) as i64
    } else {
        (num - num % 1.0 - 1.0) as i64
    }
}
