/*
   В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
   считать  неотрицательное целое число (u64) в переменную seed и вывести количество положительных
   и отрицательных чисел в массиве array в виде сообщений:

       Положительных чисел: {}
       Отрицательных чисел: {}
   Переменная seed - это начальное значение для генератора случайных чисел, чтобы генерируемая
   последовательность чисел была детерминированной и повторяемой.

   Нахождение и вывод положительных и отрицательных чисел реализуйте в соответствующей функции
   calc_pos_neg() передав в нее массив array.
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
    // Установка seed
    let seed: u64 = input::<u64>();

    // Создание массива размером 1_000_000, не трогать
    let mut array = [0; 1_000_000];

    // Инициализация генератора случайных чисел, не трогать
    let mut rng = Lcg::new(seed);

    // Заполнение массива случайными числами в диапазоне от -10 до 10, не трогать
    for i in 0..array.len() {
        array[i] = rng.gen_range(-10, 10);
    }

    calc_pos_neg(array);
}

fn calc_pos_neg(arr: [i32; 1000000]) {
    let mut positive_num: u64 = 0;
    let mut negative_num: u64 = 0;
    for num in arr.iter() {
        if *num > 0 {
            positive_num += 1;
        } else if *num < 0 {
            negative_num += 1;
        }
    }
    println!("Положительных чисел: {}", positive_num);
    println!("Отрицательных чисел: {}", negative_num)
}
