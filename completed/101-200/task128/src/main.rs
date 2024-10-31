/*
    Напишите программу, которая считывает целое число x (i8), извлекает его младший установленный
    бит и выводит получившееся число в двоичной и десятичной записи в виде сообщений:

    извлеченный из {x} младший установленный бит
    в двоичной записи: {:b}
    в десятичной записи: {}
    
    Если у числа нет установленных бит, вывести: у числа {:b} нет установленных бит.
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
    let n: i8 = input::<i8>();
    if n == 0 {
        println!("у числа {:08b} нет установленных бит", n)
    } else {
        println!("извлеченный из {n} младший установленный бит");
        println!("в двоичной записи: {:08b}", n & !(n - 1));
        println!("в десятичной записи: {}", n & !(n - 1));
    }
}
