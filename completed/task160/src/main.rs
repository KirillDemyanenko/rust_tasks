/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать в массив arr 10 целых чисел и строку ord, задающую порядок сортировки:

        asc - сортировка по возрастанию;
        dec - сортировка по убыванию;
        
    После вывести отсортированную коллекцию с помощью заполнителя {:?}.
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
    println!("{:?}", sort(arr, &*input::<String>()));
}

fn sort(mut arr: [i32; 10], ord: &str) -> [i32; 10] {
    for i in 0..arr.len() - 1 {
        for j in i..arr.len() {
            if ord == "asc" {
                if arr[i] > arr[j] {
                    arr.swap(i, j);
                }
            } else {
                if arr[i] < arr[j] {
                    arr.swap(i, j);
                }
            }
        }
    }
    arr
}

