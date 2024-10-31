/*
    Напишите программу, которая считывает 10 целых чисел и выводит все повторяющиеся значения
    через пробел по убыванию. Если повторяющихся чисел нет, вывести: Повторяющихся чисел нет.
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
    let mut arr: [i32; 10] = [0; 10];
    let mut tmp: Vec<i32> = Vec::new();
    for i in 0..arr.len() {
        arr[i] = input::<i32>();
    }
    arr.sort();
    arr.reverse();
    for i in 0..arr.len() {
        if arr.iter().filter(|&&x| x == arr[i]).count() > 1 && !tmp.contains(&arr[i]) {
            tmp.push(arr[i]);
        }
    }
    if tmp.len() == 0 {
        println!("Повторяющихся чисел нет");
    } else {
        println!("{}", tmp.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}
