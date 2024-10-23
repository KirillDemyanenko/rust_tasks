/*
    В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна
    считать длину слова (u8) и размерность алфавита (u8), а затем вывести количество слов, которые можно составить из заданного алфавита.

    Нахождение количества слов реализуйте в функции word_count() ! Результирующее значение должно
    быть типа u64.
*/
use std::fmt::Debug;
use std::str::FromStr;

pub fn input<T: FromStr>() -> T
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
    println!("{}", word_count(input::<u8>(), input::<u8>()))
}

fn word_count(word_len: u8, alph_len: u8) -> u64 {
    (word_len as f64).powf(alph_len as f64) as u64
}
