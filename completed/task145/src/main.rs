/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать строчное значение sep и 10 вещественных чисел в массив и вывести (до 2 знаков)
   получившуюся коллекцию, элементы которой разделены строкой sep.

    Нахождение количества слов реализуйте в функции print_sep_arr() ! Поскольку метод trim() удаляет
    из строки и пробелы, а тему строк ещё не прошли, в редакторе приведен метод replace() удаляющий
    только символ новой строки \n.
*/

use std::fmt::{format, Debug};
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
    let mut sep = <String>::new();
    std::io::stdin()
        .read_line(&mut sep)
        .expect("Input error!");
    let mut arr: [f64; 10] = [0.0; 10];
    for i in 0..arr.len() {
        arr[i] = input::<f64>();
    }
    print_sep_arr(arr, sep);
}

fn print_sep_arr(mut arr: [f64; 10], mut sep: String) {
    sep = sep.replace("\n", "");
    for i in 0..arr.len() - 1 {
        print!("{:.2}{sep}", arr[i]);
    }
    println!("{:.2}", arr[arr.len() - 1]);
}
