/*
    В редакторе кода представлена неполная программа, которую необходимо
    доделать. Программа должна считать радиус шара (f64) и вывести (до 3 знаков
    его объем:
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
    let r: f64 = input::<f64>();
    println!("{:.3}", std::f64::consts::PI * r.powf(3.0) * 4.0 / 3.0)
}
