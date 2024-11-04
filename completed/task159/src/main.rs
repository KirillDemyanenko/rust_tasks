/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать в массив 10 целых чисел и вывести максимальное и минимальное значения в виде сообщений:
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
    let arr: [i32; 10] = [0; 10].map(|_| input::<i32>());
    println!("max:{}", max(arr));
    println!("min:{}", min(arr));
}

fn max(array: [i32; 10]) -> i32 {
    let mut res = array[0];
    for i in 1..10 {
        if res < array[i] {
            res = array[i];
        }
    }
    res
}

fn min(array: [i32; 10]) -> i32 {
    let mut res = array[0];
    for i in 1..10 {
        if res > array[i] {
            res = array[i];
        }
    }
    res
}
