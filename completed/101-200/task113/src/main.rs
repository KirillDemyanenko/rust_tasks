/*
    Напишите программу, которая считывает натуральное число n (u8) и выводит ромб из * со сторонами
    равными n.
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
    let mut spaces:i8 = (n - 1) as i8;
    let mut stars: i8 = 1;
    for _ in 0..n {
        println!("{}{}", " ".repeat(spaces as usize), "*".repeat(stars as usize));
        stars += 2;
        if spaces > 0 {
            spaces -= 1;
        }
    }
    stars = ((n + 1) * 2 - 3) as i8;
    spaces = 0;
    for _ in 0..n - 1 {
        stars -= 2;
        spaces += 1;
        println!("{}{}", " ".repeat(spaces as usize), "*".repeat(stars as usize));
    }
}
