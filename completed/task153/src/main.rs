/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считывать натуральные (u32) числа до тех пор пока не будет введено 0, а затем вывести (до 3
   знаков) для них среднюю гармоническую.
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
    let mut numbers: Vec<i32> = Vec::new();
    let mut user_input = input::<i32>();
    while user_input != 0 {
        numbers.push(user_input);
        user_input = input::<i32>();
    }
    let mut h: f64 = 0.0;
    for i in &numbers {
        h += 1.0 / *i as f64;
    }
    print_harm_mean(numbers.len() as f64 / h);
}

fn print_harm_mean(harm_mean: f64) {
    println!("{:.3}", harm_mean);
}
