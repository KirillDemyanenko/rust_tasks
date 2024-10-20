/*
     редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
     считывать натуральное число n (u32) и столько же раз вызывать функцию print_str() из main()
     при этом выводя номер вызова:
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

fn print_str(i: u32) {
    print!("{i}) была вызвана функция!\n");
}

fn main() {
    for i in 1..=input::<u32>() {
        print_str(i);
    }
}