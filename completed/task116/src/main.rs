/*
    Напишите программу, которая считывает натуральное число n (u8) и выводит числовой треугольник, 
    состоящий из последовательности строк. В каждой строке числа увеличиваются по порядку, начиная
    с 1, и добавляется одно число с каждой новой строкой.
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
    let n: u8 = input::<u8>();
    for i in 1..=n {
        for j in 1..=i {
            print!("{j}{}", if j != i { " " } else { "" });
        }
        println!();
    }
}
