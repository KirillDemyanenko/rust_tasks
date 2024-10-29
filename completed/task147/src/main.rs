/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать часы от 0 до 23 и вывести в формате {hour} am / pm / midnight / noon, как показано в
   примерах.

   Вывод реализуйте в функции twelve_h_clck() !
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
    let mut arr: [i32; 20] = [0; 20];
    for i in 0..arr.len() {
        arr[i] = input::<i32>();
    }
    println!(
        "[{}]",
        norm_arr(arr)
            .iter()
            .map(|&num| format!("{:.2}", num))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn norm_arr(array: [i32; 20]) -> [f64; 20] {
    let min = *array.iter().min().unwrap() as f64;
    let max = *array.iter().max().unwrap() as f64;
    let mut result = [0.0; 20];
    if min == max {
        return result
    }
    for i in 0..array.len() {
        result[i] = (array[i] as f64 - min) / (max - min)
    }
    result
}
