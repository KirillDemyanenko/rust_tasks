/*
    Напишите программу, которая считывает 10 целых чисел и выводит наибольшую возрастающую
    подпоследовательность. Если последовательностей несколько вывести первую найденную.
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
    let mut start = 0;
    let mut end = 0;
    let mut vec = Vec::new();
    for i in 0..arr.len() {
        arr[i] = input::<i32>();
        if i > 0 {
            if arr[i - 1] < arr[i] {
                end += 1;
            } else { 
                vec.push(arr[start..=end].to_vec());
                start = i;
                end = i;
            }
        }
    }
    vec.push(arr[start..=end].to_vec());
    start = 0;
    end = 0;
    for i in 0..vec.len() {
        if vec[i].len() > start {
            start = vec[i].len();
            end = i;
        }
    }
    for i in 0..vec[end].len() {
        if i < vec[end].len() - 1 {
            print!("{} ", vec[end][i]);
        } else {
            println!("{}", vec[end][i]); 
        }
    }
}
