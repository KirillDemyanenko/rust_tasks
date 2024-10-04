/*
    Напишите программу, которая считывает в два массива по десять целых чисел и
    выводит результат поэлементного сравнения.

    Если массивы равны, вывести: Массивы {:?} и {:?} равны.
    Иначе вывести Массивы {:?} и {:?} не равны.
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
    return buffer.trim().parse::<T>().expect("Parse error");
}

fn main() {
    let mut arr1: [i32; 10] = [0; 10];
    let mut arr2: [i32; 10] = [0; 10];
    for i in 0..20 {
        if i < 10 {
            arr1[i] = input::<i32>();
        } else {
            arr2[i - 10] = input::<i32>();
        }
    }
    println!(
        "Массивы {arr1:?} и {arr2:?} {}равны",
        if arr1 == arr2 { "" } else { "не " }
    )
}
