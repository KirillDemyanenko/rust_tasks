/*
    Напишите программу, которая считывает три положительных целых числа (день 1-31 u8, месяц 1-12
    u8 и год 1-2024 u16) и выводит могут ли считанные числа представлять календарную дату.

    Если могут вывести: Дата корректна!
    Иначе вывести: Дата некорректна!
    Учитывайте, что в зависимости от года (високосный или нет) количество дней в некоторых месяцах
    также меняется.
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
    let day: u8 = input::<u8>();
    let month: u8 = input::<u8>();
    let year: u16 = input::<u16>();
    if (1 <= day && day <= 31) || (1 <= month && month <= 12) || (1 <= year && year <= 2024) {
        if month == 2 && year % 4 == 0 && day <= 29 {
            if year % 100 == 0 {
                if year % 400 != 0 && day == 29 {
                    println!("Дата некорректна!")
                } else {
                    println!("Дата корректна!")
                }
            } else {
                println!("Дата корректна!")
            }
        } else {
            if [1, 3, 5, 7, 8, 10, 12].contains(&month) && day <= 31 {
                println!("Дата корректна!")
            } else if [4, 6, 9, 11].contains(&month) && day <= 30 {
                println!("Дата корректна!")
            } else if month == 2 && day <= 28 {
                println!("Дата корректна!")
            } else {
                println!("Дата некорректна!")
            }
        }
    } else {
        println!("Дата некорректна!")
    }
}