/*
    Напишите программу, которая считывает три целых числа:

     - частоту дискретизации (кГц) с которой кодируется аудиопоток в режиме
        стерео (2 канала);
     - пропускную способность (Кбайт/сек);
     - объем сжатия в %, позволяющий сократить объём передаваемой информации.

    И выведите максимальную глубину кодирования в виде целого числа, если
    аудиопоток кодируется в режиме стерео (2 канала)
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
    let sample_rate: u32 = input::<u32>();
    let throughput: u32 = input::<u32>();
    let compression_volume: u32 = input::<u32>();
    println!(
        "Максимальная глубина кодирования: {}",
        (throughput * 1024 * 8) / (2 * sample_rate * 1000 * (100 - compression_volume) / 100)
    );
}
