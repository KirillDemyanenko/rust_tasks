/*
    Биатлонист N раз стреляет по мишеням. Вероятность попадания в мишень при одном выстреле равна P.
    Известно также, что биатлонист первые C раз попал в мишени, а оставшиеся промахнулся.

    Напишите программу, которая считывает числа N (u8), P (f64) и C (u8), а затем выводит (до 2
    знаков) вероятность того, что биатлонист первые C раз попал в мишени, а последние промахнулся.
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
    let p: f64 = input::<f64>();
    let c: u8 = input::<u8>();
    let mut result = 1_f64;
    for _ in 0..c {
        result *= p;
    }
    for _ in 0..n - c {
        result *= 1_f64 - p;
    }
    println!("{:.2}", result);
}
