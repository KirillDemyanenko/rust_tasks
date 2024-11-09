/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать два вещественных числа (радианы и градусы соответственно) и вывести (до 7 знаков) для
   радиан градусы, а для градусов радианы.
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
    print_degrees(input::<f64>());
    print_radians(input::<f64>());
}

fn calc_degrees(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

fn print_degrees(degrees: f64) {
    println!("{:.7}", calc_degrees(degrees));
}

fn calc_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

fn print_radians(radians: f64) {
    println!("{:.7}", calc_radians(radians));
}
