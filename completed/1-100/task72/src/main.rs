/*
    Звуковой фрагмент был записан в формате квадро (четырёхканальная запись)
    оцифрован и сохранён в виде файла без использования сжатия данных. Затем
    тот же звуковой фрагмент был записан повторно в формате моно и оцифрован с
    разрешением в 2 раза выше и частотой дискретизации в 1.5 раза меньше, чем в
    первый раз. Сжатие данных не производилось.

    Напишите программу, которая считывает целое число, являющееся размером
    полученного файла без учёта размера заголовка файла  (в Мбайт), а затем
    выводит размер файла (Мбайт) полученного при повторной записи, округлив до
    целого числа.

    Единицу измерения писать не нужно. Искомый объём не учитывает размера
    заголовка файла.
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
    let mut sum = 0.0;
    let stop = input::<String>();
    let mut current = input::<String>();
    while stop != current {
        sum += current.parse::<f64>().unwrap();
        current = input::<String>();
    }
    println!("{sum:.1}");
}
