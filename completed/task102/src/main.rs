/*
    Напишите программу, которая считывает целое многозначное целое число (u32) и выводит,
    расположены ли его цифры в порядке возрастания.

    Если цифры расположены по возрастанию, вывести Цифры числа {} расположены по возрастанию;
    Иначе вывести Цифры числа {} не расположены по возрастанию.
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
    let mut n: u32 = input::<u32>();
    print!("Цифры числа {n} ");
    let mut prev = n % 10;  
    while n > 0 {
        n /= 10;
        if prev <= n % 10 {
            print!("не ");
            break
        }
        prev = n % 10;
    }
    println!("расположены по возрастанию");
}