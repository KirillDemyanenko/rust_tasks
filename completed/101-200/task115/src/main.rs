/*
    Напишите программу, которая считывает натуральное число n (u8) и выводит пару чисел (i, j), где
    i начинается с 0, j начинается с 1, и каждое последующее значение i и j увеличивается на 2. 
    Программа должна продолжать выводить пары чисел до тех пор, пока значение i не достигнет или
    превысит n.
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
    let n: u8 = input::<u8>();
    'main: for i in (0..=n).step_by(2) {
        for j in (i + 1)..=n {
            println!("{i} {j}");
            continue 'main;
        }
    }
}
