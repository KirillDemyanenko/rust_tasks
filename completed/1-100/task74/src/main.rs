/*
    Напишите программу, которая считывает количество часов (u8) отработанных за неделю, а затем
    выводит (до 2 знаков) заработную плату до вычетов и после, а также сумму налогов, учитывая:

     - базовый почасовой тариф: 1500 руб/час.
     - переработку более 40 часов в неделю: базовый почасовой тариф * 1.5.
     - налоговая ставка: 13%.
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
    let hours: i8 = input::<i8>();
    let mut  all_money = hours as f64 * 1500.00;
    if hours > 40 {
        all_money = 40.0 * 1500.0 + (hours as f64 - 40.0) * (1500.00 * 1.5);
    }
    println!("Заработная плата до вычетов: {:.2} руб", all_money);
    println!("Сумма налогов: {:.2} руб", all_money * 0.13);
    println!("Заработная плата после вычетов: {:.2} руб", all_money * 0.87);
}
