/*
    Напишите программу, которая считывает натуральное число n (u8) и выводит треугольник из * с высотой равным n.
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
    let n = input::<u8>();
    let mut spaces = (n - 1) as i8;
    let mut stars = 1;
    for _ in 0..n {
        println!("{}{}", " ".repeat(spaces as usize), "*".repeat(stars as usize));
        stars += 2;
        spaces -= 1;
    }
}
