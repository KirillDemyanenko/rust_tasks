/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать неотрицательное целое число (u32) и вывести его факториал. Факториал натурального числа 
   n определяется как произведение всех натуральных чисел от 1 до n включительно:
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
    println!("{}", factorial(input::<u32>()));
}

fn factorial(n: u32) -> u32 {
    let mut fact = 1;
    for i in 1..=n {
        fact *= i
    }
    fact
}
