/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать год (u16) и вывести его римский эквивалент
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
    print_rome(input::<u16>());
}

fn print_rome(mut year: u16) {
    let mut rome = String::new();
    while year > 0 {
        match year {
            1000_u16..=u16::MAX => {
                year -= 1000;
                rome = [&*rome, "M"].join("").to_string()
            }
            900..=999 => {
                year -= 900;
                rome = [&*rome, "CM"].join("").to_string()
            }
            500..=899 => {
                year -= 500;
                rome = [&*rome, "D"].join("").to_string()
            }
            400..=499 => {
                year -= 400;
                rome = [&*rome, "CD"].join("").to_string()
            }
            100..=399 => {
                year -= 100;
                rome = [&*rome, "C"].join("").to_string()
            }
            90..=99 => {
                year -= 90;
                rome = [&*rome, "XC"].join("").to_string()
            }
            50..=89 => {
                year -= 50;
                rome = [&*rome, "L"].join("").to_string()
            }
            40..=49 => {
                year -= 40;
                rome = [&*rome, "XL"].join("").to_string()
            }
            10..=39 => {
                year -= 10;
                rome = [&*rome, "X"].join("").to_string()
            }
            9 => {
                year -= 9;
                rome = [&*rome, "IX"].join("").to_string()
            }
            5..=8 => {
                year -= 5;
                rome = [&*rome, "V"].join("").to_string()
            }
            4 => {
                year -= 4;
                rome = [&*rome, "IV"].join("").to_string()
            }
            1..=3 => {
                year -= 1;
                rome = [&*rome, "I"].join("").to_string()
            }
            0 => year = 0,
        }
    }
    println!("{}", rome);
}
