/*
    Напишите программу, которая считывает десять целых чисел, а затем выводит с помощью {:?}
    получившуюся последовательность по возрастанию.
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
    for i in 0..arr.len() {
        arr[i] = input::<i32>();
    }
    arr.sort();
    println!("{:?}", arr);
}
